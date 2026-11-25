> Pergunta: "quais unidades de código o `wyd-client` tem, qual é a API de cada uma e o que ela não pode violar?"
> Público: Desenvolvedor do `wyd-client`
> Dono: equipe do wyd-client
> Base: `wyd-client-1.0.0.md` §1, §2, §3, §6; `AGENTS.md` §6; `wyd-admin/docs/arquitetura/componentes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum arquivo de código existe. A árvore abaixo descreve unidades de responsabilidade, não arquivos 1:1.

## `build-identity`

- **Responsabilidade:** medir versão e hash do binário do cliente (build-alvo 7662 + SHA-256).
- **API:** `identify() -> BuildInfo`.
- **Invariantes:** build-alvo fixado; comparação entre builds segue não medida.
- **Depende de:** nenhum.
- **Testes:** hash estável; versão declarada.
- **Risco:** médio.

## `capture-tool`

- **Responsabilidade:** ferramenta específica em `wyd-client/tools/` (CLI Rust) que orquestra captura, normaliza evidência.
- **API:** `capture(scenario) -> CaptureResult`.
- **Invariantes:** escopo estreito; não implementa proxy/servidor/protocolo; só ambiente controlado; sem dado de jogador.
- **Depende de:** `wyd-server` (ECAP), `wyd-protocol`.
- **Testes:** captura em ambiente sintético; recusa se surgir dado real.
- **Risco:** alto — captura é prova.

## `anonymization`

- **Responsabilidade:** remover conta, nome, IP e chat antes de gravar.
- **API:** `anonymize(raw) -> frame`.
- **Invariantes:** nenhum dado pessoal em artefato versionado; anonimizar não substitui prevenção.
- **Depende de:** nenhum.
- **Testes:** frame sem PII; varredura de log.
- **Risco:** alto — privacidade.

## `ficha`

- **Responsabilidade:** ficha de captura: data, build, cenário, o que mediu, operador, QA.
- **API:** `write_ficha(capture) -> Ficha`.
- **Invariantes:** sem ficha, a captura é registro, não prova; segunda leitura por pessoa diferente.
- **Depende de:** `build-identity`, `anonymization`.
- **Testes:** ficha completa; segunda leitura assinada.
- **Risco:** médio.

## `golden`

- **Responsabilidade:** registrar golden `G-<familia>-<nn>` com origem declarada.
- **API:** `register_golden(frame)`.
- **Invariantes:** origem datada; nível de confiança; golden nunca inventado.
- **Depende de:** `ficha`, `wyd-protocol`.
- **Testes:** golden com origem; rejeição sem origem.
- **Risco:** alto — é a prova do fio.

## `patch-content`

- **Responsabilidade:** conteúdo do patch e `strdef.bin` (lista fechada em `registro-do-cliente.md` §5.1; `LCLI-02` fechada em 2026-11-25).
- **API:** `list_patch_files() -> Vec<File>`.
- **Invariantes:** patch descreve exatamente os arquivos; sem redistribuição sem autorização.
- **Depende de:** `wyd-launcher` (contrato).
- **Testes:** lista candidata; verificação de hash.
- **Risco:** médio.

## `ip-rights`

- **Responsabilidade:** frente de direitos: inventário, pacote do pedido, usos pretendidos.
- **API:** `inventory()`, `request_package()`.
- **Invariantes:** distribuição congelada; nenhum conteúdo publicado sem autorização.
- **Depende de:** `docs/specs/propriedade-intelectual-e-distribuicao.md`.
- **Testes:** inventário completo; pacote não enviado.
- **Risco:** alto — legal.

## `http`

- **Responsabilidade:** cliente HTTP para portal/manifesto.
- **API:** `get(url) -> bytes`.
- **Invariantes:** timeout; sem segredo.
- **Depende de:** nenhum.
- **Testes:** timeout; erro de rede.
- **Risco:** baixo.

## Árvore de código e de teste

```text
src/                      # planejamento; código real mora em wyd-client/ (raiz) ou tools/
├── build-identity/       # versão + hash
├── capture-tool/         # CLI Rust (wyd-client/tools/)
├── anonymization/        # remoção de PII
├── ficha/                # ficha de captura
├── golden/               # registro de golden
├── patch-content/        # conteúdo do patch
├── ip-rights/            # frente de direitos
└── http/                 # cliente HTTP
tests/
├── unitario/            # build-identity, anonymization
├── contrato/            # golden, ficha
├── integracao/          # captura local
├── sistema/             # (n/a)
├── e2e/                 # Playwright (AGENTS.md §8)
└── propriedade/         # fuzz de manifesto
```

*Página de planejamento. Nenhuma unidade foi implementada.*