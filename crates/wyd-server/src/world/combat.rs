//! Submódulo de `world` (ADR-0026, etapa 3).

use super::*;

impl Engine {
    /// Golpe corpo a corpo autoritativo no ponto clicado.
    ///
    /// `Ok(None)` = não há criatura de combate no alcance (o cliente não escolhe alvo fora
    /// do alcance). O dano sai de [`TemplateDamage`] sobre os `Stats` das duas entidades; o
    /// cliente só diz **onde** golpeou. Abate devolve a EXP do template, que a sessão aplica
    /// com o multiplicador do snapshot.
    pub fn attack_at(
        &mut self,
        attacker: Entity,
        x: i32,
        y: i32,
        now_ms: u64,
    ) -> Result<Option<AttackOutcome>, CoreError> {
        let Some((entity, npc_id, exp, template)) =
            self.npc_near(x, y, ATTACK_RANGE_TILES, true).map(|npc| {
                (
                    npc.entity,
                    npc.spec.id,
                    npc.spec.exp,
                    npc.spec.template.clone(),
                )
            })
        else {
            // Sem criatura no ponto: com PvP ligado, o alvo pode ser outro jogador.
            return self.attack_player(attacker, x, y, now_ms);
        };
        self.strike(attacker, entity, npc_id, exp, template, now_ms)
    }

    /// Golpe no `MobID` de cena que veio em `Dam[i].TargetID` do `_MSG_Attack`.
    ///
    /// É a rota do `MobAttack` do legado (`Server.cpp:9881`: `int Target =
    /// sm.Dam[i].TargetID;`): o alvo é o **id do pacote**, não o tile. O cliente 7662 preenche
    /// `Dam[].TargetID` e pode mandar `TargetX/TargetY` zerado no golpe básico — resolver só
    /// pelo ponto clicado devolvia `result=no_target` e o jogador não tirava HP de ninguém.
    /// O alcance continua sendo a posição autoritativa do motor, nunca a do cliente.
    pub fn attack_scene_at(
        &mut self,
        attacker: Entity,
        scene: u16,
        now_ms: u64,
    ) -> Result<Option<AttackOutcome>, CoreError> {
        let Some((entity, npc_id, exp, template)) = self
            .npcs
            .values()
            .find(|npc| self.scene_ids.get(&npc.entity.index).copied() == Some(scene))
            .map(|npc| {
                (
                    npc.entity,
                    npc.spec.id,
                    npc.spec.exp,
                    npc.spec.template.clone(),
                )
            })
        else {
            return Ok(None);
        };
        let (Some(posicao_atacante), Some(posicao_alvo)) = (
            self.store.position_of(attacker),
            self.store.position_of(entity),
        ) else {
            return Ok(None);
        };
        if posicao_atacante
            .x
            .abs_diff(posicao_alvo.x)
            .max(posicao_atacante.y.abs_diff(posicao_alvo.y))
            > ATTACK_RANGE_TILES
        {
            return Ok(None);
        }
        self.strike(attacker, entity, npc_id, exp, template, now_ms)
    }

    /// Aplica o golpe numa criatura já resolvida (por tile ou por `MobID`) e devolve o
    /// resultado de domínio; a sessão cuida de EXP/loot.
    fn strike(
        &mut self,
        attacker: Entity,
        entity: Entity,
        npc_id: i32,
        exp: i64,
        template: String,
        now_ms: u64,
    ) -> Result<Option<AttackOutcome>, CoreError> {
        // Id de cena **antes** do despawn: `despawn` devolve o id ao pool, e ler depois dava
        // `target_scene = 0` no golpe que mata — o eco `_MSG_AttackOne` e o `_MSG_RemoveMob`
        // saíam com id 0 e o cliente não tirava o cadáver nem desenhava o dano final.
        let target_scene = self.scene_ids.get(&entity.index).copied().unwrap_or(0);
        let golpe = combat_system::attack(
            &mut self.store,
            attacker,
            entity,
            now_ms,
            ATTACK_RULES,
            &TemplateDamage,
        )?;
        let killed = golpe.remaining_hp == 0;
        // Morte real: a criatura sai do mundo no mesmo comando. Deixá-la no mapa com HP 0
        // seria um cadáver que o cliente desenha e o jogador não consegue mais golpear; a
        // geração sobe para a borda emitir `_MSG_RemoveMob` (tipo morte). O loot do abate é
        // resolvido na sessão, com a tabela do snapshot e o sorteio do momento.
        if killed {
            self.despawn(entity);
        }
        Ok(Some(AttackOutcome {
            target_npc: npc_id,
            target_scene,
            template,
            damage: golpe.amount,
            remaining_hp: golpe.remaining_hp,
            killed,
            exp: if killed { exp } else { 0 },
            target_actor: false,
            cp_after: 0,
            cp_delta: 0,
        }))
    }

    /// Golpe em **outro jogador** (PvP) no ponto clicado.
    ///
    /// Só existe com o PvP do canal/mapa ligado e autor com CP acima do piso (`-75`). O
    /// abate desconta o caos do autor (0 para vítima já listada) e devolve a vítima com HP
    /// cheio — sem EXP/item, porque no original o CP é a consequência do kill. O save
    /// periódico leva o CP novo (`LiveState.cp`).
    fn attack_player(
        &mut self,
        attacker: Entity,
        x: i32,
        y: i32,
        now_ms: u64,
    ) -> Result<Option<AttackOutcome>, CoreError> {
        if !self.pvp {
            return Ok(None);
        }
        let atacante_cp = self.live.get(&attacker.index).map_or(0, |state| state.cp);
        if !wyd_sim::pode_iniciar_pk(atacante_cp) {
            return Ok(None);
        }
        // Menor distância e depois menor índice: mesma ordem determinística de `npc_near`.
        let alvo = self
            .by_character
            .values()
            .copied()
            .filter(|entity| *entity != attacker)
            .filter_map(|entity| {
                let posicao = self.store.position_of(entity)?;
                let hp = self.store.stats_of(entity)?.hp;
                let distancia = posicao.x.abs_diff(x).max(posicao.y.abs_diff(y));
                (hp > 0 && distancia <= ATTACK_RANGE_TILES).then_some((
                    distancia,
                    entity.index,
                    entity,
                ))
            })
            .min_by_key(|(distancia, indice, _)| (*distancia, *indice))
            .map(|(_, _, entity)| entity);
        let Some(alvo) = alvo else {
            return Ok(None);
        };
        let golpe = combat_system::attack(
            &mut self.store,
            attacker,
            alvo,
            now_ms,
            ATTACK_RULES,
            &TemplateDamage,
        )?;
        let killed = golpe.remaining_hp == 0;
        let mut cp_after = atacante_cp;
        let mut cp_delta = 0;
        if killed {
            let vitima_cp = self.live.get(&alvo.index).map_or(0, |state| state.cp);
            cp_delta = wyd_sim::penalidade_de_kill(vitima_cp);
            cp_after = wyd_sim::clamp_cp(atacante_cp - cp_delta);
            if let Some(state) = self.live.get_mut(&attacker.index) {
                state.cp = cp_after;
            }
            if let Some(stats) = self.store.stats_of(alvo) {
                self.store.restore_hp(alvo, stats.max_hp);
                if let Some(state) = self.live.get_mut(&alvo.index) {
                    state.hp = stats.max_hp;
                }
            }
        }
        Ok(Some(AttackOutcome {
            target_npc: 0,
            target_scene: self.scene_ids.get(&alvo.index).copied().unwrap_or(0),
            template: String::new(),
            damage: golpe.amount,
            remaining_hp: golpe.remaining_hp,
            killed,
            exp: 0,
            target_actor: true,
            cp_after,
            cp_delta,
        }))
    }
}

/// Dano do canal: `attack` do atacante contra o `defense` do alvo pela
/// `BASE_GetDamage` do legado (`Basedef.cpp:1265`, transcrita em
/// `docs/wyd/pesquisa-wyd2-missao-fase2-2026-11-10.md` §4.1) — AC mitiga **metade** do valor
/// e a escada de pisos evita golpe nulo.
///
/// A variância da arma (`rnd = rand()%delta + combat + 99`) fica de fora porque este modelo é
/// determinístico: o fator aplicado é 100% (o centro da faixa 99..110). O sorteio entra na
/// FASE-10, junto com maestria/crítico/perfuração.
struct TemplateDamage;

impl DamageModel for TemplateDamage {
    fn damage(&self, attacker: Stats, defender: Stats) -> u32 {
        legacy_damage(attacker.attack, defender.defense)
    }
}

/// `BASE_GetDamage(dam, ac, combat)` com fator percentual neutro.
///
/// Fonte: `Basedef.cpp:1265-1294` — `tdam = dam - ac/2`; fator da arma em `[combat+99, 110]`%
/// (aqui 100%); escada de pisos `tdam < -50 → 0`, `-50..0 → (tdam+50)/7`,
/// `0..=50 → 5*tdam/4 + 7`; dano mínimo 1.
fn legacy_damage(dam: u32, ac: u32) -> u32 {
    let mut tdam = i64::from(dam) - i64::from(ac / 2);
    if tdam < -50 {
        tdam = 0;
    } else if tdam < 0 {
        tdam = (tdam + 50) / 7;
    } else if tdam <= 50 {
        tdam = 5 * tdam / 4 + 7;
    }
    u32::try_from(tdam.max(1)).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::legacy_damage;

    /// `Basedef.cpp:1265`: o AC do alvo entra pela metade e a escada de pisos devolve 5 mesmo
    /// quando `dam - ac/2` fica negativo — é o que faz um nível 1 arranhar um alvo de AC 40.
    #[test]
    fn escada_do_legado_com_ac_pela_metade() {
        assert_eq!(legacy_damage(5, 40), 5, "5 - 20 = -15 → (35)/7 = 5");
        assert_eq!(legacy_damage(5, 2), 12, "5 - 1 = 4 → 5*4/4 + 7 = 12");
        assert_eq!(legacy_damage(10, 0), 19, "10 → 5*10/4 + 7 = 19");
        assert_eq!(legacy_damage(0, 0), 7, "0 → 7 (piso da faixa 0..=50)");
        assert_eq!(
            legacy_damage(1, 200),
            1,
            "abaixo de -50 cai em 0 e o mínimo é 1"
        );
    }
}
