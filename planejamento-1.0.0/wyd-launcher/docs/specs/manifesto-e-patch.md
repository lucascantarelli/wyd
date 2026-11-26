# Manifesto e patch — formato, canonização, assinatura e verificação

> Pergunta: "como o manifesto do patch é escrito, canonizado, assinado e verificado antes de o launcher tocar no cliente?"
> Público: desenvolvimento do launcher, `wyd-client` (produz o patch), `wyd-ops` (assina e publica), segurança
> Dono: equipe do `wyd-launcher`
> Base: `docs/arquitetura/adr/adr-lch-0001-manifesto-canonico-assinado.md`; `lacunas.md` `LLCH-01`/`LLCH-03`; `docs/specs/patch-1.0.0.json`; `crates/wyd-launcher/src/manifest.rs`; `crates/wyd-launcher/src/verify.rs`; `wyd-client/registro-do-cliente.md` §5.1
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). O formato e a ordem de verificação estão fixados pelo `ADR-LCH-0001`. **Têm código** a leitura de forma, a conferência de hash e a canonização RFC 8785
(`crates/wyd-launcher/src/canon.rs`, vetor do RFC em `tests/rfc8785.rs`); a assinatura Ed25519 não
tem código nem fixture — este descritor diz o que falta, não afirma que existe.

Este é o descritor que o `ADR-LCH-0001` nomeia. Ele descreve o **envelope**: como o manifesto é escrito, o que é assinado, em que ordem o launcher verifica e onde cada regra mora no código. **O conteúdo do patch não é deste dono:** quais arquivos a `1.0.0` entrega está fechado por `LCLI-02` (opção A, 2026-11-25) em `wyd-client/registro-do-cliente.md` §5.1, e o manifesto medido apenas o transcreve.

## 1. Formato

O manifesto é um arquivo JSON UTF-8 com seis campos de topo e uma lista de arquivos.

| Campo | Tipo | Regra | Origem |
|---|---|---|---|
| `schema` | inteiro | exatamente `1`; valor novo exige código novo, não fallback | `SCHEMA` em `manifest.rs` |
| `channel` | string | `stable` ou `beta`, os mesmos de `portal.launcher_release.channel` | `CHANNELS` em `manifest.rs` |
| `version` | string | não vazio; é o semver da release | `Manifest::validate` |
| `client_build` | string | SHA-256 do build do cliente: 64 dígitos hex minúsculos | `Manifest::validate` |
| `min_version` | string | não vazio; versão mínima que aceita o patch | `Manifest::validate` |
| `files[]` | lista | não vazia; cada item tem `path`, `size`, `sha256` e, opcionalmente, `url` | `FileEntry` |
| `files[].path` | string | relativo à raiz do cliente, separador `/`, sem `..`, sem raiz absoluta, sem letra de unidade, sem `\` | `is_relative_path` |
| `files[].size` | inteiro | tamanho em bytes | `FileEntry` |
| `files[].sha256` | string | 64 dígitos hex minúsculos | `is_sha256` |
| `files[].url` | string | ausente ou `https://…`; ausente enquanto não há host de release publicado | `Manifest::validate` |

A ordem de `files` não é validada pelo código; o manifesto medido lista em ordem de caminho. Um caminho repetido reprova antes de qualquer leitura de disco, porque dois registros do mesmo arquivo tornam a aplicação ambígua.

### Recusas de forma

Toda recusa acontece **antes** de o launcher abrir um arquivo do cliente, e cada uma tem variante própria em `ManifestError`:

| Recusa | Variante | Gatilho |
|---|---|---|
| JSON inválido | `Json` | texto que não desserializa |
| schema desconhecido | `Schema` | `schema` diferente de `1` |
| canal desconhecido | `Channel` | valor fora de `stable`/`beta` |
| campo vazio | `Empty` | `version` ou `min_version` em branco |
| build inválido | `ClientBuild` | `client_build` que não é SHA-256 |
| manifesto sem arquivos | `NoFiles` | `files` vazio |
| caminho inválido | `Path` | absoluto, com `..`, com `\` ou com `:` |
| caminho repetido | `DuplicatePath` | dois itens com o mesmo `path` |
| hash inválido | `Sha256` | hash com tamanho errado ou maiúsculas |
| URL inválida | `Url` | `url` que não começa por `https://` |

## 2. Canonização

A assinatura cobre **bytes canônicos**, não o arquivo como ele foi editado. O `ADR-LCH-0001` fixa **RFC 8785 (JCS)**: UTF-8 sem BOM, sem espaço insignificante, chaves ordenadas por unidade de código UTF-16, números na forma do `Number::toString` do ECMAScript e sem quebra de linha final.

Consequência que o produtor precisa respeitar: o mesmo manifesto reindentado tem a mesma forma
canônica — a assinatura continua válida —, mas trocar um `1.0.0` por `1.0.1`, reordenar a lista ou
trocar um hash muda os bytes canônicos e invalida a assinatura.

**Implementada em 2026-11-25** em `crates/wyd-launcher/src/canon.rs`: `canonicalize` e
`canonicalize_str` ordenam chave por unidade de código UTF-16, escapam só o que a RFC manda e
formatam número pela regra do ECMAScript; inteiro acima de 2^53 e valor não finito são recusados em
vez de arredondados. O vetor normativo da RFC está em `tests/rfc8785.rs`. Detalhe que a implementação
obrigou: sem a feature `float_roundtrip`, o `serde_json` lê `333333333.33333329` como o `f64` vizinho
errado (1 ulp, `…54` em vez de `…55`) e o vetor do RFC falha — a feature está ligada no `Cargo.toml`
do crate.

**Ponto ainda não fixado pelo ADR:** se o launcher **recusa** bytes não canônicos ou **normaliza** o documento recebido antes de verificar. As duas leituras preservam a garantia (a assinatura é sobre a forma canônica), e a escolha muda a mensagem de erro e o teste de borda; ela precisa ser tomada antes da primeira assinatura real.

## 3. Assinatura

- **Algoritmo:** Ed25519, assinatura **destacada** — arquivo irmão `manifest.json.sig` do `manifest.json`.
- **Cobertura:** os bytes canônicos do §2, nada mais; nome do arquivo, data e canal fora do JSON não são assinados.
- **Chave pública e `key_id`:** embutidos no launcher, em `config` (`docs/arquitetura/componentes.md`).
- **Chave privada:** só no secret do `wyd-ops`; nunca no binário, no repositório nem no manifesto.
- **Rotação:** exige launcher novo com a nova pública (ou lista de chaves) e não exige migration; `portal.launcher_release.manifest_sha256` continua sendo o registro de publicação.

**Verificação implementada em 2026-11-25** em `crates/wyd-launcher/src/signature.rs`
(`TrustStore::verify` e `verificar`): canonicaliza o manifesto, decodifica o `.sig`, confere com a
chave pública Ed25519 do `key_id` e falha em `SignatureError` (`UnknownKey`, `SignatureEncoding`,
`KeyEncoding`, `PublicKey`, `Invalid`, `Canon`). Fixture real em `tests/fixtures/assinatura/`
(manifesto, `.sig`, chave pública e chave errada). O **encoding** dos 64 bytes, que o ADR deixou para
a primeira implementação, ficou fixado: **hexadecimal minúsculo** com 128 caracteres, espaço ou
quebra de linha nas pontas tolerados; a chave pública usa a mesma forma, 64 caracteres.

**Assinador:** `crates/wyd-ops-signer` (`wyd-ops-sign sign --manifest <arq> --out <arq.sig>`), com
`WYD_LAUNCHER_SIGNING_KEY` ou `--key-file`; reaproveita `wyd_launcher::canonicalize_str`, não tem
canonização própria. O passo de release que o executa está em `.github/workflows/release.yml` e falha
antes de assinar quando o secret `WYD_LAUNCHER_SIGNING_KEY` não existe. **A chave privada continua fora
do repositório:** o que existe aqui é o código que a lê do secret e o teste com semente de teste.

## 4. Ordem de verificação

O launcher não aplica nada antes de a origem estar provada. A ordem é parte do contrato, porque verificar depois de escrever não protege nada:

| # | Passo | Mora em | Estado (2026-11-25) |
|---|---|---|---|
| 1 | baixar `manifest.json` e `manifest.json.sig` por HTTPS, com timeout | `http` | não implementado |
| 2 | canonizar o manifesto (RFC 8785) | `canon` | **implementado** (falta ligar ao passo 3) |
| 3 | verificar a assinatura contra a pública do `key_id` | `signature` | **implementado** |
| 4 | desserializar e validar a forma (§1) | `Manifest::from_json` | **implementado** |
| 5 | conferir `size` e `sha256` de cada arquivo sob a raiz do cliente | `verify` | **implementado** |
| 6 | aplicar o patch só com relatório sem divergência | `patch` | não implementado |

A validação de forma vem **depois** da assinatura no desenho: um manifesto não autenticado não chega ao passo 4, e nenhuma regra de forma é aplicada antes de a origem estar provada. Hoje o crate expõe os passos 4 e 5 isolados — é o que `LCLI-02` exigia e o que os testes cobrem.

### Relatório de integridade

`verify(manifest, root)` devolve `Report { checked, divergences }` e nunca escreve. Cada arquivo declarado cai em um de quatro resultados:

- **ok:** tamanho e `sha256` batem; não entra no relatório.
- **`Missing`:** o caminho não existe sob a raiz.
- **`Size`:** o tamanho medido difere do declarado; o hash não chega a ser comparado.
- **`Hash`:** tamanho igual, `sha256` diferente.

Divergência é **resultado**, não erro de execução: o launcher recusa o patch inteiro e diz qual arquivo divergiu. Falha de I/O que não seja "arquivo ausente" sobe como `io::Error`.

## 5. O que prova que este descritor está atendido

| Critério | Comando ou artefato | Estado |
|---|---|---|
| forma do manifesto aceita e recusada nos dez casos | `cargo test -p wyd-launcher --all-targets` | verde (27 testes de unidade) |
| patch real sem divergência contra `wyd-client/files` | `crates/wyd-launcher/tests/patch_1_0_0.rs` | verde |
| canonização estável e igual ao vetor RFC 8785 | `crates/wyd-launcher/tests/rfc8785.rs` | verde (3 testes) |
| assinatura válida, inválida e chave errada | `crates/wyd-launcher/tests/assinatura.rs` | verde (7 testes) |
| ciclo completo: assinador do `wyd-ops` → `TrustStore` do launcher | `crates/wyd-launcher/tests/ciclo-assinatura.rs` | verde (4 testes) |
| schema publicado, validado contra a fixture e contra o manifesto medido | `docs/specs/manifest.schema.json` + `crates/wyd-launcher/tests/schema.rs` | verde (9 testes) |
| release assina e publica manifesto + `.sig` | `crates/wyd-ops-signer` + `.github/workflows/release.yml` (secret `WYD_LAUNCHER_SIGNING_KEY`) | **existe**; falha sem o secret |
| assinatura reprovada antes de qualquer escrita | teste de ordem | **falta** (depende do passo 6, aplicar patch) |

`LLCH-01` e `LLCH-03` seguem fechadas **como decisão** e implementadas nos dois lados — schema,
canonização, verificação, assinador e passo de release. Fica aberto **apenas** o teste de ordem
"assinatura reprovada antes de qualquer escrita": ele precisa do passo 6 (aplicar patch), que ainda
não tem código; nenhuma release é publicada com este manifesto antes disso.

## 6. Limites deste descritor

- Não afirma manifesto publicado, host de release nem tamanho de patch: nenhum artefato foi publicado.
- O encoding de `manifest.json.sig` foi fixado pela implementação (§3): hex minúsculo; fica sem
  decisão a política de bytes não canônicos (recusar ou normalizar).
- Não cobre o conteúdo do patch (`LCLI-02`, dono `wyd-client`), a hospedagem (`wyd-ops`) nem o instalador e o updater (`LLCH-04`).

## Referências

- Decisão: `ADR-LCH-0001` (`Q-LCH-1` = A e `Q-LCH-3` = A, 2026-11-23).
- Lacunas: `LLCH-01`, `LLCH-03` em `lacunas.md`.
- Manifesto medido: `docs/specs/patch-1.0.0.json`.
- Lista do patch: `wyd-client/registro-do-cliente.md` §5.1.
- Código: `crates/wyd-launcher/src/manifest.rs`, `src/canon.rs`, `src/signature.rs`, `src/verify.rs`;
  testes `tests/manifesto.rs`, `tests/rfc8785.rs`, `tests/assinatura.rs`, `tests/patch_1_0_0.rs`.
- Contrato: `X11` em `wyd-1.0.0.md` §7.3; unidade `signature` em `docs/arquitetura/componentes.md`.
