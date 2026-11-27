# Skills do `wyd-docs` — roteador

O `wyd-docs` monta e publica o site MkDocs a partir de `planejamento-1.0.0/*/docs/`. Só `docs/` vai
para o site (`D-6`); migration, skill, teste e prompt não são publicados.

| Tarefa | Skill | Quando não usar |
|---|---|---|
| publicar página nova ou mover página existente | [`execution/publicar-pagina`](execution/publicar-pagina/SKILL.md) | a página é de `docs/` de um sistema e ainda não está aceita |
| revisar o site montado (link, órfão, navegação, censo) | [`review/revisar-pagina-publicada`](review/revisar-pagina-publicada/SKILL.md) | revisão do texto da página (é `wyd-db/review/revisar-documentacao`) |

## Fronteiras

- Arquitetura aprova o sistema novo em `SISTEMAS` de `wyd-docs/preparar.py`; documentação publica.
- Não existe `mkdocs.yml` no repositório: versão, tema e build vivem em `wyd-docs/`.
