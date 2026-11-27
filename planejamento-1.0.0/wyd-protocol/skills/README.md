# Skills do `wyd-protocol` — roteador

Segundo piloto do projeto. O protocolo é derivado do **tráfego real do cliente 7662**, nunca de
suposição: pacote sem captura que o comprove fica `H`.

| Tarefa | Skill | Quando não usar |
|---|---|---|
| escrever ou alterar a spec de um pacote/opcode | [`execution/escrever-spec-de-pacote`](execution/escrever-spec-de-pacote/SKILL.md) | o pacote já tem spec aceita e nada mudou |
| provar comportamento contra o cliente | [`test/provar-contra-cliente`](test/provar-contra-cliente/SKILL.md) | a prova é só de unidade local |

## Fronteiras

- Teto de páginas em `wyd-protocol-1.0.0.md` §4; lacunas em `lacunas.md` com ID `LPRT-*`.
- Sem ADR aceito neste sistema: decisão sem "sim" do dono é proposta.
