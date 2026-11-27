# Skills do `wyd-db` — roteador

Uma skill por tipo de tarefa deste sistema. Escolha **uma** principal; se precisar de duas, a
segunda é verificação, não reescrita. O molde e a fronteira skill × prompt são de
`.agents/skills/README.md`; as regras de negócio não entram aqui — vivem em `specs/` e `docs/` e são
citadas.

| Tarefa | Skill | Quando não usar |
|---|---|---|
| transformar pedido vago em requisito `RF-DB`/`RN-DB` com critério de aceitação | [`plan/levantar-requisito`](plan/levantar-requisito/SKILL.md) | requisito já numerado em `specs/` |
| quebrar trabalho em onda com arquivos, dependência e critério de pronto | [`plan/planejar-onda`](plan/planejar-onda/SKILL.md) | issue única e óbvia |
| executar uma issue de ponta a ponta (código, teste, doc, PR) | [`execution/implementar-issue`](execution/implementar-issue/SKILL.md) | a issue é só de migration — use a skill de migration |
| escrever ou corrigir um arquivo da série `0001`–`0034` e a página dele | [`execution/escrever-migration`](execution/escrever-migration/SKILL.md) | o arquivo da série já está publicado em produção |
| revisar DDL contra o padrão do banco | [`review/revisar-schema`](review/revisar-schema/SKILL.md) | revisão de página de planejamento |
| revisar página contra cabeçalho, marcas e o que reprova | [`review/revisar-documentacao`](review/revisar-documentacao/SKILL.md) | revisão de DDL |
| provar schema: estrutural, contrato, banco real e ciclo `up`/`down` | [`test/provar-schema`](test/provar-schema/SKILL.md) | teste de comportamento de runtime |
| preparar conta e fixture de teste | [`test/provisionar-dados-de-teste`](test/provisionar-dados-de-teste/SKILL.md) | dado de conteúdo de seed |

## Regra de escrita

Toda skill tem **quando usar**, **pré-requisitos**, **passos**, **como verificar**, **quando parar e
perguntar** e **armadilhas**. Passo que não pode ser verificado não entra. Exemplo é sempre do
repositório, nunca inventado.

## Onde estas skills moram

Aqui, e só aqui: `planejamento-1.0.0/wyd-db/skills/` é a pasta reservada pelo plano diretor
(`wyd-db-1.0.0.md` §5.14 e §6.19). Os demais sistemas ainda não abriram a pergunta própria, então
skill deles não nasce em outro lugar (`estrutura-de-pastas-e-modulos.md` §2). O site não publica
`skills/` (`D-6`): se a regra precisa de leitor humano, ela sobe para `docs/`, não para cá.
