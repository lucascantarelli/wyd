# Evidências de depreciação — workspace Rust, portal Node e scripts

> Pergunta: "há warning, API ou dependência depreciada na árvore viva, e qual comando prova isso em cada área?"
> Público: desenvolvimento, arquitetura, `wyd-ops` e agentes
> Dono: `wyd-ops` (toolchain e política de dependência)
> Base: `inventario-depreciacao.tsv`; `AGENTS.md` §5; `docs/estrutura-de-pastas-e-modulos.md` §2.4; `security.yml`
> Revisado em: 2026-11-26

**Estado:** proposto (2026-11-26) — evidência medida; nenhuma página de planejamento foi aceita por causa
desta medição.

## 1. Escopo e método

A análise cobre a **árvore viva**: os 10 crates de `crates/`, o portal Next.js em `wyd-portal/` e os
scripts versionados de `wyd-ops/scripts/` e `wyd-docs/`. Ficam fora `deprecated/` (legado ignorado pelo
git, referência de nível 4), `wyd-client/files/` (binário capturado, ignorado) e os diretórios gerados
`target/`, `node_modules/`, `.next/` e `.build/`.

Cada linha do `inventario-depreciacao.tsv` registra **ID, área, comando exato, saída relevante, data e
conclusão**. O TSV é dado de evidência; esta página é o resumo que amarra as conclusões. Nenhum código
de domínio foi criado ou alterado pela medição. A única correção de configuração foi a exceção de
licença dos 10 crates em `wyd-ops/deny.toml`, descrita em §3.

## 2. Resumo consolidado

| Área | Medição | Conclusão | Evidência |
|---|---|---|---|
| Rust: APIs marcadas | busca por `#[deprecated]`/`deprecated(` nos `.rs` vivos | **0** ocorrências | DEP-02 |
| Rust: warnings | `cargo clippy --all-targets --locked` nos 10 crates, com `CARGO_TARGET_DIR` novo e vazio | **0** warning/erro; 10/10 exit 0, sem reuso de cache | DEP-03 |
| Rust: política | `cargo deny` com `wyd-ops/deny.toml` nos 10 crates | **10/10** `advisories ok, bans ok, licenses ok, sources ok` | DEP-04 |
| Rust: advisories | `cargo audit --file` nos 10 `Cargo.lock` | **0** vulnerabilidades; `found=false` | DEP-05 |
| Rust: versões duplicadas | `cargo tree --duplicates` | duplicatas transitivas existem (ex.: `syn` 2/3, `sha2` 0.10/0.11); **não** são API deprecated | DEP-06 |
| Node: dependências | `npm ls`, `npm outdated`, `npm audit --package-lock-only` | nenhum campo `deprecated`; `outdated {}`; **0** vulnerabilidades | DEP-07 |
| Scripts | `node --check`, `bash -n`, AST Python | **22/22** arquivos válidos (1 Node + 4 shell + 17 Python); `shellcheck` ausente | DEP-08 a DEP-10 |
| Legado | `git ls-files deprecated` | **0** arquivos versionados; fora da árvore viva | DEP-11 |

**Conclusão:** não há warning, API marcada como `deprecated` nem dependência declarada como depreciada
na árvore viva em 2026-11-26. As duplicatas de versão são avisos de `multiple-versions` do `cargo deny`,
não depreciação; ficam registradas para a frente de atualização de dependências, sem serem tratadas como
regra de negócio. O achado real foi de **licença**: antes da correção, os 7 crates-esqueleto reprovavam
porque `GPL-2.0-only` só estava excepcionada para `wyd-db`, `wyd-ops-signer` e `wyd-launcher`; a exceção
foi estendida aos 10 crates do projeto e a política passou.

## 3. O que foi corrigido na política

`wyd-ops/deny.toml` mantém a lista permissiva para dependências e a exceção `GPL-2.0-only` restrita aos
crates do projeto. A lista de exceções agora nomeia os 10 crates: `wyd-db`, `wyd-ops-signer`,
`wyd-launcher`, `wyd-protocol`, `wyd-net`, `wyd-core`, `wyd-sim`, `wyd-server`, `wyd-admin` e `wyd-bots`.
Isso não altera licença, código ou dependência; apenas faz a política cobrir os crates que já existem.

## 4. Captura completa do cliente

O estado não foi adiado: a condição do `registro-do-cliente.md` §7.1 foi relida e continua dependendo
de **insumo que só o usuário pode fornecer**. O que falta, exatamente:

1. **uma conta de teste em um peer que responda ao CPSock**, ou
2. **aprovação escrita para uma sessão com credencial sintética**, descartando os frames de login antes
   de qualquer artefato; e
3. o **nome do operador** na ficha de origem (`LCLI-03`), que ainda não existe.

Sem (1) ou (2), não há como produzir tráfego legítimo do cliente: a tentativa de 2026-11-26 chegou à
tela de Login sem abrir o socket CPSock e não enviou login. Sem o nome do operador, a ficha não pode ser
assinada. O restante do trabalho de captura — implementar o `ECAP`, a CLI e o golden — fica bloqueado
por esses insumos, não por falta de trabalho de repositório. Não houve nova tentativa de captura nem
gravação de credencial; `capture.wydcap` não existe.

O que **já está resolvido e validado** no repositório é o inventário do cliente: 7.147 arquivos, 567 MB,
`client.zip` e `WYD.exe` com os hashes do registro, e `wyd-client/files/` ignorado pelo git (CAP-01).
O `wyd-client/tools/.gitkeep` preserva o destino aprovado da CLI; não é captura nem comportamento novo.

## 5. Como reproduzir

```bash
# Rust: símbolos, warnings e política
grep -rn --include='*.rs' --exclude-dir=target --exclude-dir=deprecated '#\[deprecated\|deprecated(' crates
for c in wyd-db wyd-ops-signer wyd-launcher wyd-protocol wyd-net wyd-core wyd-sim wyd-server wyd-admin wyd-bots; do
  cargo clippy --manifest-path "crates/$c/Cargo.toml" --all-targets --locked --message-format=short
  cargo deny --config wyd-ops/deny.toml --manifest-path "crates/$c/Cargo.toml" check advisories bans licenses sources
  cargo audit --file "crates/$c/Cargo.lock" --json
done

# Portal
cd wyd-portal
npm ls --depth=0
npm outdated --json
npm audit --package-lock-only --json

# Scripts
node --check wyd-ops/scripts/sync-issue-status.mjs
for f in wyd-ops/scripts/ci/*.sh wyd-docs/gate.sh; do bash -n "$f"; done
```

O comando completo por linha, a saída e a data estão em
`planejamento-1.0.0/wyd-ops/inventario-depreciacao.tsv` (dado de evidência, não página publicada). A
captura e seus bloqueios estão em `planejamento-1.0.0/wyd-client/registro-do-cliente.md` §7.1.

## 6. Limites

- `shellcheck` não está instalado nesta estação; a evidência de shell é `bash -n`, não análise semântica.
- `cargo clippy` foi reexecutado com `CARGO_TARGET_DIR` novo e vazio: os 10 crates recompilaram do zero e
  ainda assim 10/10 saíram com exit 0 e nenhuma linha `warning:`/`error:`; não há cache mascarando diagnóstico.
- A análise não transforma `deprecated/` em fonte: o legado continua ignorado e fora das conclusões.
- A captura do cliente continua bloqueada pelos insumos de §4; nenhuma credencial foi usada ou gravada.
