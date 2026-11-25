> Pergunta: "como o `wyd-protocol` prova o layout do fio sem inventar byte?"
> Público: QA e desenvolvedor do `wyd-protocol`
> Dono: equipe do wyd-protocol
> Base: `AGENTS.md` §8; `wyd-protocol-1.0.0.md` §6.8, §8, §9; `wyd-admin/docs/planos/plano-de-testes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum teste escrito.

## Pirâmide e alvos

| Nível | Alvo | O que prova | Dado |
|---|---|---|---|
| Unitário | `wire`, `codec`, `error` | offsets, limites, checksum, transformação, taxonomia | funções puras, sem I/O |
| Contrato | `tests/contrato/` | catálogo de opcodes e golden por família | gerador + fixtures |
| Integração | `tests/propriedade/` | roundtrip encode/decode, faixa, fuzz | bytes gerados |
| Sistema | `tests/golden/` | bytes provam layout com origem declarada | golden `G-<familia>-<nn>` |
| E2E | `wyd-net`/`wyd-bots` | fio ponta a ponta com o codec real | ambiente efêmero |
| Propriedade | `tests/propriedade/` | fuzz de decode sem panic | corpus gerado |

E2E do protocolo é Rust, coerente com `AGENTS.md` §8: quem tem navegador é o portal.

## O que cada teste precisa provar

1. **Nenhum byte afirmado sem prova:** toda afirmação de layout aponta para golden `G-<familia>-<nn>` ou captura datada.
2. **Nenhum panic em decode:** fuzz de decode não panica em nenhum input até o limite.
3. **Checksum divergente é recusado:** sem modo tolerante (`LPRT-02`).
4. **Frame no limite:** `Size = 8192` aceito, acima recusado com erro tipado (`LPRT-07`).
5. **Opcode desconhecido:** devolve `UnknownType`, não queda (`LPRT-08`).
6. **Roundtrip:** encode(decode(x)) == x para todo frame válido.
7. **Gerador reprodutível:** saída determinística, diff no CI (`LPRT-16`).

## Isolamento

- Testes Rust com `cargo test`; um banco/contêiner não é necessário para o codec (puro).
- Fixtures por factory de bytes; nenhum id mágico.
- Proibido `sleep` como sincronização.
- Nenhum estado global entre testes; nenhum `#[ignore]` sem issue dona.
- Cobertura mínima de 85% por capacidade, não por linha (`LPRT-15`); código gerado fica fora da métrica de linha.

## Comandos

```bash
cargo test -p wyd-protocol --all-targets
cargo llvm-cov -p wyd-protocol --fail-under-lines 85
```

O `--fail-under-lines 85` é o gate de `AGENTS.md` §8; cobertura de linha não substitui os sete comportamentos acima.

## CI, cobertura e monitoramento

- **CI:** o job `code` de `.github/workflows/code.yml` roda `cargo fmt --all -- --check`, `cargo clippy -p wyd-protocol --all-targets -- -D warnings`, `cargo test -p wyd-protocol --all-targets` e `cargo deny check`; o PR para `develop` e `main` só passa com os quatro verdes.
- **Cobertura:** `cargo llvm-cov -p wyd-protocol --fail-under-lines 85` é o piso; a cobertura de linha não substitui os comportamentos listados acima e `#[ignore]` exige issue dona e prazo.
- **Monitoramento:** Diff de golden, diff do gerador e falha de roundtrip. O crate é puro e não expõe runtime; a prova é o job de CI e o catálogo de goldens, não uma métrica de servidor.
- **Critério verificável:** o comando de teste e o gate de documentação exit 0; nenhuma falha é mascarada por `| tail`, `--no-fail-fast` ou `SKIP` silencioso.

## Critério de saída da `1.0.0`

Todos os testes de golden, roundtrip e fuzz verdes; nenhuma página de `wire/` ou `payloads/` reprovada; nenhuma lacuna de prova aberta; total do catálogo publicado com fonte e data. `LPRT-03` e `LPRT-06` continuam abertas até captura real.