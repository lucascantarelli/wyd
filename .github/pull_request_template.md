## Resumo

<!-- Duas a quatro linhas: o problema, a mudança e o resultado observável. -->

Closes #

## Contexto e escopo

- **Issue / página dona:**
- **Sistema afetado:** <!-- wyd-db, wyd-docs, wyd-launcher, wyd-ops, ... -->
- **ADR / lacuna / decisão:** <!-- ID e link, ou "não se aplica" -->
- **Fora de escopo:** <!-- o que este PR deliberadamente não faz -->

## Tipo de mudança

- [ ] Documentação de planejamento
- [ ] Site (`wyd-docs`)
- [ ] CI, scripts ou configuração
- [ ] Código de produção (só depois do gate de planejamento)
- [ ] Correção de bug ou regressão
- [ ] Refatoração sem mudança de comportamento

## Skills carregadas

- **Skill obrigatória da issue:** <!-- copie o skill_principal da página/issue -->
- **Complementares usadas:** <!-- ou "nenhuma" -->
- **Como foi verificado:** <!-- ex.: cloud_read_skill antes do primeiro edit; SKILL.md relido -->

## Como foi provado

<!-- Comando e saída real. "Testei localmente" não é prova. -->

```text
<comando>
<saída>
```

### Matriz de verificação

| Alvo | Comando | Resultado | Observação |
|---|---|---|---|
|  |  |  |  |

### O que não foi verificado

<!-- Ex.: launcher não executado em Linux; teste E2E não existe. -->

## Impacto

| Área | Afetada? | Evidência |
|---|---|---|
| Código / comportamento |  |  |
| Banco / migration |  |  |
| Documentação e links |  |  |
| CI / branch protection |  |  |
| Deploy / operação |  |  |
| Segurança / licenças |  |  |

## Segurança e dados

- [ ] Nenhum segredo, DSN, token, hash de jogador ou chave privada no diff.
- [ ] Permissões de workflow e escopo de token continuam mínimos.
- [ ] Migração ou mudança de dados tem plano de reversão.
- [ ] Dependência nova foi justificada e passou por `cargo deny` / dependency-review.

## CI/CD, deploy e rollback

- **Checks esperados:** <!-- gate-code, gate-db, gate-docs, gate-security -->
- **Deploy necessário?** <!-- não / preview / produção; provedor e janela -->
- **Rollback:** <!-- comando, tag/SHA ou "não se aplica" -->
- **Monitoramento:** <!-- log, métrica, alerta ou "não se aplica" -->

## Documentação

- [ ] O documento dono foi atualizado na mesma entrega.
- [ ] Contagens têm fonte, comando e data; nenhuma foi copiada de outro documento.
- [ ] Links relativos resolvem e o `gate-docs` passa.
- [ ] ADR só foi criado se houver decisão registrada com "como verificar".

## Checklist do autor

- [ ] O diff contém apenas arquivos do escopo da issue.
- [ ] O launcher foi validado apenas no ambiente suportado (Windows).
- [ ] Não há `TODO`, `TBD`, placeholder ou teste silenciosamente ignorado.
- [ ] O histórico de commits explica o "porquê", não apenas o "o quê".
- [ ] A descrição acima permite revisar sem abrir o código.

## Para o revisor

<!-- Pontos de atenção, decisões que exigem aprovação e como revisar. -->
