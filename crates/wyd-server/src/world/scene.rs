//! Submódulo de `world` (ADR-0026, etapa 3).

use super::*;

impl Engine {
    /// Reserva o id de cena da entidade no espaço do legado.
    ///
    /// `actor = true` pega da faixa de jogador (`1..MAX_USER`), senão da faixa de
    /// criatura (`MAX_USER..MAX_MOB`). Id já reservado devolve o mesmo (idempotente).
    /// `None` = espaço esgotado; quem chamou desfaz o spawn.
    pub(super) fn alloc_scene_id(&mut self, entity: Entity, actor: bool) -> Option<u16> {
        if let Some(id) = self.scene_ids.get(&entity.index) {
            return Some(*id);
        }
        let id = if actor {
            match self.free_player_ids.pop() {
                Some(id) => id,
                None => {
                    let id = self.next_player_scene;
                    if id >= MAX_USER {
                        return None;
                    }
                    self.next_player_scene += 1;
                    id
                }
            }
        } else {
            match self.free_mob_ids.pop() {
                Some(id) => id,
                None => {
                    let id = self.next_mob_scene;
                    if id >= MAX_MOB {
                        return None;
                    }
                    self.next_mob_scene += 1;
                    id
                }
            }
        };
        self.scene_ids.insert(entity.index, id);
        Some(id)
    }

    /// Devolve o id de cena ao espaço de onde ele veio.
    fn release_scene_id(&mut self, entity: Entity) {
        let Some(id) = self.scene_ids.remove(&entity.index) else {
            return;
        };
        if id < MAX_USER {
            self.free_player_ids.push(id);
        } else {
            self.free_mob_ids.push(id);
        }
    }

    /// Spawn autoritativo: entra no armazém e na grade, na mesma transação de memória.
    ///
    /// Se a posição está fora do mapa, a entidade **não** fica no armazém (rollback explícito).
    pub fn spawn(&mut self, spec: SpawnSpec) -> Result<Entity, CoreError> {
        let posicao = spec.position;
        if !(0..WORLD_WIDTH).contains(&posicao.x) || !(0..WORLD_HEIGHT).contains(&posicao.y) {
            return Err(CoreError::OutOfBounds {
                x: posicao.x,
                y: posicao.y,
            });
        }
        let entity = self
            .store
            .spawn(spec.kind, posicao, spec.stats, Inventory::default())?;
        if let Err(error) = self.grid.insert(entity, posicao.x, posicao.y) {
            self.store.despawn(entity);
            return Err(error);
        }
        // Entidade viva = mudança de cena (o cliente precisa de `_MSG_CreateMob`).
        self.scene_generation = self.scene_generation.wrapping_add(1);
        // Entidade nova nasce na versão 0 (nenhuma sessão a viu ainda).
        self.revisions.remove(&entity.index);
        Ok(entity)
    }

    /// Remove a entidade do armazém **e** da grade (posição lida antes do despawn).
    pub fn despawn(&mut self, entity: Entity) -> bool {
        if let Some(posicao) = self.store.position_of(entity) {
            let _ = self.grid.remove(entity, posicao.x, posicao.y);
        }
        self.live.remove(&entity.index);
        self.by_character.retain(|_, atual| *atual != entity);
        self.npcs.retain(|_, npc| npc.entity != entity);
        self.skins.remove(&entity.index);
        self.revisions.remove(&entity.index);
        self.release_scene_id(entity);
        let saiu = self.store.despawn(entity);
        if saiu {
            // Saída = mudança de cena (o cliente precisa de `_MSG_RemoveMob`).
            self.scene_generation = self.scene_generation.wrapping_add(1);
        }
        saiu
    }

    /// Cena atual: NPCs e personagens vivos, com a posição que o motor conhece, ordenados por id.
    ///
    /// A ordem determinística importa para o delta da borda: duas leituras iguais devolvem a
    /// mesma lista, então a sessão só emite o que realmente mudou.
    #[must_use]
    /// Foto da cena **visível** a até `radius` tiles de `center` (Chebyshev).
    ///
    /// Inclui o próprio personagem (distância 0) e só as criaturas dentro do raio — é o
    /// conjunto que a sessão entrega, em vez dos 6 110 NPCs do mapa inteiro.
    pub fn scene_around(&self, center: Position, radius: u32) -> SceneSnapshot {
        let mut cena = self.scene_snapshot();
        cena.mobs.retain(|mob| {
            mob.position
                .x
                .abs_diff(center.x)
                .max(mob.position.y.abs_diff(center.y))
                <= radius
        });
        cena
    }

    pub fn scene_snapshot(&self) -> SceneSnapshot {
        let mut mobs: Vec<SceneMob> = Vec::with_capacity(self.npcs.len() + self.live.len());
        for npc in self.npcs.values() {
            let Some(position) = self.store.position_of(npc.entity) else {
                continue;
            };
            let Some(id) = self.scene_ids.get(&npc.entity.index).copied() else {
                continue;
            };
            mobs.push(SceneMob {
                id,
                actor: false,
                npc_id: npc.spec.id,
                name: fixed_name(&npc.spec.name),
                position,
                level: npc.spec.stats.level,
                hp: npc.spec.stats.hp,
                max_hp: npc.spec.stats.max_hp,
                attack: npc.spec.stats.attack,
                defense: npc.spec.stats.defense,
                // NPC do mapa não tem mana nem atributos de personagem no snapshot.
                mp: 0,
                max_mp: 0,
                strength: 0,
                intelligence: 0,
                dexterity: 0,
                constitution: 0,
                merchant: u8::try_from(npc.spec.merchant).unwrap_or(0),
                chaos_rate: crate::personagens::CP_NEUTRO,
                attack_run: npc.spec.attack_run,
                direction: npc.spec.direction,
                equip: npc.spec.equip,
                guild: 0,
                // NPC não muda de nível/atributo em jogo: nunca pede refresh.
                revision: 0,
            });
        }
        for (indice, estado) in &self.live {
            let Some(id) = self.scene_ids.get(indice).copied() else {
                continue;
            };
            let skin = self.skins.get(indice).cloned().unwrap_or_default();
            // Stats vêm do armazém (fonte autoritativa); a entidade é achada pelo personagem
            // porque `LiveState` guarda o id do banco, não a entidade.
            let stats = self
                .by_character
                .get(&estado.character_id)
                .and_then(|entity| self.store.stats_of(*entity));
            mobs.push(SceneMob {
                id,
                actor: true,
                npc_id: 0,
                name: skin.name,
                position: Position {
                    x: estado.x,
                    y: estado.y,
                },
                // `CreateMob` é fio, e no fio o `Level` é 0-based (`wire_level`: banco 1 → 0)
                // como no `CNFCharacterLogin`; `LiveState.level` guarda o nível do banco.
                // Sem esta conversão o cliente desenhava "Nv 2" para personagem nível 1.
                level: estado.level.saturating_sub(1),
                hp: estado.hp,
                max_hp: stats.map_or(estado.hp, |stats| stats.max_hp),
                attack: stats.map_or(0, |stats| stats.attack),
                defense: stats.map_or(0, |stats| stats.defense),
                // MP corrente vem do estado vivo (gasto no mundo); máximo e atributos do
                // snapshot do login, que é o mesmo `STRUCT_SCORE` do `CNFCharacterLogin`.
                mp: i32::try_from(estado.mp).unwrap_or(i32::MAX),
                max_mp: stats.map_or(skin.max_mp, |stats| {
                    i32::try_from(stats.max_mp).unwrap_or(i32::MAX)
                }),
                strength: skin.strength,
                intelligence: skin.intelligence,
                dexterity: skin.dexterity,
                constitution: skin.constitution,
                merchant: 0,
                chaos_rate: wyd_sim::chaos_rate(estado.cp),
                attack_run: skin.attack_run,
                direction: skin.direction,
                equip: skin.equip,
                guild: skin.guild,
                revision: self.revisions.get(indice).copied().unwrap_or(0),
            });
        }
        mobs.sort_by_key(|mob| mob.id);
        SceneSnapshot {
            generation: self.scene_generation,
            mobs,
        }
    }

    /// Uso de chave num portão (`_MSG_UpdateItem` do legado): ponto exato + alcance.
    ///
    /// `keys` são os `EF_KEYID` das provas que o personagem carrega (a borda lê o
    /// inventário); a chave só é apagada depois do aceite, pelo `SelectionStore`.
    pub fn use_key_at(
        &mut self,
        speaker: Entity,
        x: i32,
        y: i32,
        keys: &[i32],
        now_ms: u64,
    ) -> Result<crate::gameplay::GateOutcome, crate::gameplay::GateError> {
        use crate::gameplay::{gate_at, GateError, CAMP_GATE_RANGE};
        let portao = gate_at(x, y).ok_or(GateError::NoGate)?;
        let posicao = self
            .store
            .position_of(speaker)
            .ok_or(GateError::OutOfRange)?;
        if posicao.x.abs_diff(x).max(posicao.y.abs_diff(y)) > CAMP_GATE_RANGE as u32 {
            return Err(GateError::OutOfRange);
        }
        self.gates.open(portao, keys, now_ms)
    }

    /// Portão aberto agora (`now_ms` do processo).
    #[must_use]
    pub fn gate_open(&self, gate: i16, now_ms: u64) -> bool {
        self.gates.is_open(gate, now_ms)
    }

    /// Encarna o personagem persistido como entidade `Player` na posição salva.
    ///
    /// Toda validação acontece **antes** do spawn: dado inconsistente devolve erro e o
    /// personagem continua fora do mundo (nada de entidade com HP 0 ou posição inválida).
    /// A `skin` é a aparência do `STRUCT_MOB` do login: sem ela o cliente desenharia um
    /// personagem sem nome nem equipamento.
    pub fn enter(
        &mut self,
        character: &Character,
        skin: ActorSkin,
    ) -> Result<Entered, EnterWorldError> {
        if self.by_character.contains_key(&character.id) {
            return Err(EnterWorldError::AlreadyInWorld {
                character_id: character.id,
            });
        }
        let stats = stats_from(character, &skin)?;
        let x = i32::try_from(character.pos_x)
            .map_err(|_| EnterWorldError::InvalidState(format!("pos_x {}", character.pos_x)))?;
        let y = i32::try_from(character.pos_y)
            .map_err(|_| EnterWorldError::InvalidState(format!("pos_y {}", character.pos_y)))?;
        let entity = self
            .spawn(SpawnSpec {
                kind: wyd_core::EntityKind::Player,
                position: wyd_core::Position { x, y },
                stats,
            })
            .map_err(EnterWorldError::Core)?;
        // Faixa de jogador (`< MAX_USER`): é o número que vai em `ClientID`/`MobID`.
        let Some(scene_id) = self.alloc_scene_id(entity, true) else {
            self.despawn(entity);
            return Err(EnterWorldError::SceneFull);
        };
        let state = LiveState {
            character_id: character.id,
            level: stats.level,
            cp: i16::try_from(character.cp).unwrap_or(0).clamp(-75, 75),
            exp: character.exp,
            hp: stats.hp,
            mp: stats.mp,
            x,
            y,
        };
        self.live.insert(entity.index, state);
        self.skins.insert(entity.index, skin);
        self.by_character.insert(character.id, entity);
        Ok(Entered { entity, scene_id })
    }

    /// Aplica no mundo vivo o nível/atributos que o banco acabou de gravar.
    ///
    /// Não recalcula fórmula: copia o `STRUCT_SCORE` do personagem para o armazém
    /// (level, HP/MP) e para a aparência (MP e atributos) e sobe a `revision`. Ataque/defesa
    /// ficam como estão — level up não os altera neste servidor e trocá-los faria o
    /// `CreateMob` divergir do `CNFCharacterLogin` do próprio dono. As outras sessões
    /// comparam a versão e reenviam o `CreateMob` — é o level up/ponto de atributo sem
    /// relogin. `false` = entidade fora do mundo.
    pub fn refresh_actor(&mut self, entity: Entity, refresh: ActorRefresh) -> bool {
        if !self.live.contains_key(&entity.index) || !self.skins.contains_key(&entity.index) {
            return false;
        }
        let Some(atual) = self.store.stats_of(entity) else {
            return false;
        };
        let stats = Stats {
            level: refresh.level,
            hp: refresh.hp,
            max_hp: refresh.max_hp,
            mp: refresh.mp,
            max_mp: refresh.max_mp,
            attack: atual.attack,
            defense: atual.defense,
        };
        if stats.validate().is_err() || !self.store.set_stats(entity, stats) {
            return false;
        }
        if let Some(state) = self.live.get_mut(&entity.index) {
            state.level = refresh.level;
            state.hp = refresh.hp;
            state.mp = refresh.mp;
        }
        if let Some(skin) = self.skins.get_mut(&entity.index) {
            skin.mp = i32::try_from(refresh.mp).unwrap_or(i32::MAX);
            skin.max_mp = i32::try_from(refresh.max_mp).unwrap_or(i32::MAX);
            skin.strength = refresh.strength;
            skin.intelligence = refresh.intelligence;
            skin.dexterity = refresh.dexterity;
            skin.constitution = refresh.constitution;
        }
        let revision = self.next_revision.max(1);
        self.next_revision = revision.wrapping_add(1);
        self.revisions.insert(entity.index, revision);
        true
    }

    /// Tira o personagem do mundo e devolve o estado final (o chamador decide salvar).
    pub fn leave(&mut self, entity: Entity) -> Option<LiveState> {
        let state = self.live.remove(&entity.index);
        if let Some(state) = state {
            self.by_character.remove(&state.character_id);
            self.despawn(entity);
        }
        state
    }
}

/// Calcula o delta entre o que a sessão já viu (`sent`: id → papel + versão) e a foto atual.
///
/// Três casos: entidade nova (`enter`), entidade que saiu (`leave`) e entidade que mudou
/// nível/atributos (`refresh`). Posição/HP em combate não entram: o pacote de movimento não
/// está provado (GAME-09), mas o `CreateMob` reemitido **com os dados novos** é a única forma
/// de outro jogador ver o level up sem relogin — e o cliente recusa `MobID` repetido, então a
/// sessão manda `RemoveMob` + `CreateMob` (dois pacotes; o teto conta os dois).
///
/// Saídas vêm primeiro: liberar o cliente de um fantasma vale mais que desenhar um novo.
#[must_use]
pub fn plan_scene(
    sent: &std::collections::BTreeMap<u16, SentMob>,
    snapshot: &SceneSnapshot,
    budget: usize,
) -> ScenePlan {
    let atuais: std::collections::BTreeMap<u16, &SceneMob> =
        snapshot.mobs.iter().map(|mob| (mob.id, mob)).collect();
    let mut leave: Vec<(u16, i32)> = sent
        .iter()
        .filter(|(id, _)| !atuais.contains_key(id))
        .map(|(id, visto)| {
            (
                *id,
                if visto.actor {
                    REMOVE_LOGOUT
                } else {
                    REMOVE_OUT_OF_VIEW
                },
            )
        })
        .collect();
    leave.sort_unstable_by_key(|(id, _)| *id);
    let mut refresh: Vec<SceneMob> = snapshot
        .mobs
        .iter()
        .filter(|mob| {
            sent.get(&mob.id)
                .is_some_and(|visto| visto.revision != mob.revision)
        })
        .cloned()
        .collect();
    let mut enter: Vec<SceneMob> = snapshot
        .mobs
        .iter()
        .filter(|mob| !sent.contains_key(&mob.id))
        .cloned()
        .collect();
    let total = leave.len() + enter.len() + refresh.len() * 2;
    let complete = total <= budget;
    if !complete {
        // Saídas primeiro, depois atualizações (2 pacotes cada), depois entradas.
        let mut restante = budget;
        leave.truncate(restante);
        restante = restante.saturating_sub(leave.len());
        refresh.truncate(restante / 2);
        restante = restante.saturating_sub(refresh.len() * 2);
        enter.truncate(restante);
    }
    ScenePlan {
        enter,
        refresh,
        leave,
        complete,
    }
}
