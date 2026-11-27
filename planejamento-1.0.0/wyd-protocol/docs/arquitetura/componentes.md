> Pergunta: "quais unidades de código o `wyd-protocol` tem, qual é a API de cada uma e o que ela não pode violar?"
> Público: Desenvolvedor do `wyd-protocol`
> Dono: equipe do wyd-protocol
> Base: `wyd-protocol-1.0.0.md` §1.4, §5.2, §6.2; `wyd-admin/docs/arquitetura/componentes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum arquivo de código existe. O crate é Rust; a árvore abaixo descreve unidades de responsabilidade, não arquivos 1:1.

## `wire`

- **Responsabilidade:** representar o `HEADER` de 12 bytes, enquadramento por `Size`, limites de tamanho e flags de direção.
- **API:** `Header { size, keyword, checksum, type_id, id, client_tick }`; `Frame`; leitura/escrita explícita offset a offset, sem `unsafe`.
- **Invariantes:** `Size` mínimo e máximo com origem declarada; pacote parcial/colado/lixo no buffer tratados como estados, não panic; `HEADER_SIZE = 12` congelado.
- **Depende de:** `error`.
- **Testes:** golden de header; enquadramento de pacote parcial e colado; limite `Size = 8192`.
- **Risco de alteração:** alto — mudar offset quebra o fio inteiro.

## `codec`

- **Responsabilidade:** transformar bytes em frame e frame em bytes: deofuscação, checksum, decodificação de `Type`.
- **API:** `decode(buf) -> Frame`, `encode(frame) -> bytes`, `checksum(buf) -> u8`.
- **Invariantes:** checksum divergente é recusado (sem modo tolerante, `LPRT-02`); codec é stateless; sem I/O; sem `unsafe`; erro tipado, nunca panic.
- **Depende de:** `wire`, `error`, `generated`.
- **Testes:** roundtrip; checksum inválido; transformação com exemplo numérico.
- **Risco:** alto — é onde o fio é provado.

## `payload`

- **Responsabilidade:** decodificar e codificar o payload de cada mensagem por família, com faixa e nulidade.
- **API:** um tipo por mensagem; `decode(bytes) -> Payload`, `encode(payload) -> bytes`.
- **Invariantes:** payload tipado e validado; bytes crus não saem do crate (`LPRT-18`); campo sem proveniência reprova; string é bytes de tamanho fixo, sem transcodificação (`LPRT-06`).
- **Depende de:** `wire`, `error`, `generated`.
- **Testes:** golden por família; faixa violada; campo opcional ausente.
- **Risco:** alto — cada família é uma superfície de compatibilidade.

## `error`

- **Responsabilidade:** taxonomia de erro (`HeaderError`, `TransformError`, `PayloadError`) com classe recuperável/fatal.
- **API:** enum tipado; conversão para código de reação sugerido.
- **Invariantes:** codec classifica, `wyd-net` decide a reação; `UnknownType` não causa queda por si só; nenhum `SQLSTATE`/panic cru.
- **Depende de:** nenhum módulo de negócio.
- **Testes:** mapeamento de cada variante; classe recuperável/fatal.
- **Risco:** médio — duplicar taxonomia cria duas políticas.

## `generated`

- **Responsabilidade:** tabelas geradas (`Opcode`, `KEY_TABLE`) e sua reprodutibilidade.
- **API:** `Opcode` enum; `KEY_TABLE`; gerador próprio com entrada versionada e saída determinística.
- **Invariantes:** gerado fica fora da métrica de linha (`LPRT-15`); matriz de capacidade por variante; diff no CI; total medido do gerador, nunca herdado do legado (198 é piso).
- **Depende de:** nenhum (dados).
- **Testes:** reprodutibilidade do gerador; diff no CI; cobertura de capacidade.
- **Risco:** alto — tabela divergente quebra o fio.

## Árvore de código e de teste

```text
src/                      # planejamento; código real mora em wyd-protocol/ (raiz) ou crates/wyd-protocol/
├── wire/                 # HEADER, enquadramento, limites, flags
├── codec/                # decode/encode, checksum, transformação
├── payload/              # payloads por família
├── error/                # taxonomia de erro
└── generated/            # Opcode, KEY_TABLE e gerador
tests/
├── unitario/            # codec, wire, error (Rust)
├── contrato/            # catálogo e golden (Rust)
├── propriedade/         # roundtrip e fuzz (Rust)
└── golden/              # bytes de prova (G-<familia>-<nn>)
```

*Página de planejamento. Nenhuma unidade foi implementada.*
