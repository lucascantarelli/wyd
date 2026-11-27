> Pergunta: "quais unidades de código o `wyd-launcher` tem, qual é a API de cada uma e o que ela não pode violar?"
> Público: Desenvolvedor do `wyd-launcher`
> Dono: equipe do wyd-launcher
> Base: `wyd-launcher-1.0.0.md` §1, §2, §3, §6; `AGENTS.md` §6; `wyd-admin/docs/arquitetura/componentes.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Nenhum arquivo de código existe. A árvore abaixo descreve unidades de responsabilidade, não arquivos 1:1.

## `manifest`

- **Responsabilidade:** ler e validar o manifesto JSON canônico (schema, channel, version, client_build, min_version, files).
- **API:** `load_manifest(url) -> Manifest`, `validate(manifest)`.
- **Invariantes:** canonização RFC 8785; hash por arquivo; manifesto adulterado não aplica patch.
- **Depende de:** `signature`, `http`.
- **Testes:** manifesto válido/inválido; hash divergente.
- **Risco:** alto — é o contrato do patch.

## `signature`

- **Responsabilidade:** verificar assinatura Ed25519 destacada antes de aplicar.
- **API:** `TrustStore::verify(key_id, manifest_json, manifest_json.sig) -> Result<(), SignatureError>`;
  o `.sig` é hex minúsculo de 64 bytes (`crates/wyd-launcher/src/signature.rs`).
- **Invariantes:** chave pública + `key_id` no launcher; chave privada só no `wyd-ops`; verificação antes de aplicar.
- **Depende de:** config.
- **Testes:** assinatura válida/inválida; chave errada; ciclo completo no
  `crates/wyd-launcher/tests/ciclo-assinatura.rs`, assinado pelo `crates/wyd-ops-signer`.
- **Outro lado:** o assinador vive no `wyd-ops` (`crates/wyd-ops-signer`, binário `wyd-ops-sign`),
  lê a chave do secret `WYD_LAUNCHER_SIGNING_KEY` e não tem canonização própria.
- **Risco:** alto — segurança do patch.

## `patch`

- **Responsabilidade:** aplicar patch publicado: baixar, verificar hash, substituir arquivo.
- **API:** `apply(manifest) -> Report`.
- **Invariantes:** arquivo divergente não inicia; sem segredo no binário.
- **Depende de:** `manifest`, `integrity`, `http`.
- **Testes:** patch aplicado; falha de hash aborta.
- **Risco:** alto — altera arquivos do cliente.

## `integrity`

- **Responsabilidade:** conferir integridade dos arquivos locais contra o manifesto.
- **API:** `check(files) -> Vec<Divergence>`.
- **Invariantes:** hash conferido; divergente não inicia.
- **Depende de:** `manifest`.
- **Testes:** arquivo ok/divergente/ausente.
- **Risco:** médio.

## `channels`

- **Responsabilidade:** obter lista de canais e estado do servidor (portal/publicação + cache local).
- **API:** `list_channels() -> Vec<Channel>`.
- **Invariantes:** canal fora do ar não aparece como disponível; cache local.
- **Depende de:** `http`, `cache`.
- **Testes:** lista com cache; canal em manutenção oculto.
- **Risco:** médio.

## `news`

- **Responsabilidade:** buscar notícias e links do portal.
- **API:** `fetch_news() -> Vec<News>`.
- **Invariantes:** nenhum conteúdo embutido no binário.
- **Depende de:** `http`.
- **Testes:** fetch ok/falha.
- **Risco:** baixo.

## `updater`

- **Responsabilidade:** atualizar o próprio launcher (NSIS + updater Ed25519 via `wyd-ops`).
- **API:** `check_update()`, `apply_update()`.
- **Invariantes:** publicado em GitHub Releases; reusa L1/L5.
- **Depende de:** `manifest`, `signature`.
- **Testes:** update disponível/indisponível.
- **Risco:** médio.

## `client-launch`

- **Responsabilidade:** iniciar o cliente após verificação de integridade.
- **API:** `launch(client_path)`.
- **Invariantes:** só após integridade ok; passa argumentos de canal.
- **Depende de:** `integrity`, `channels`.
- **Testes:** launch bloqueado se divergente.
- **Risco:** médio.

## `http`

- **Responsabilidade:** cliente HTTP para manifesto, canais, notícias, patch.
- **API:** `get(url) -> bytes`.
- **Invariantes:** timeout; sem segredo no binário.
- **Depende de:** nenhum.
- **Testes:** timeout; erro de rede.
- **Risco:** baixo.

## `cache`

- **Responsabilidade:** cache local de canais/notícias.
- **API:** `get`, `set`, `invalidate`.
- **Invariantes:** invalidação coerente.
- **Depende de:** nenhum.
- **Testes:** cache hit/miss.
- **Risco:** baixo.

## `config`

- **Responsabilidade:** URL do portal, chave pública, `key_id`.
- **API:** `load() -> Config`.
- **Invariantes:** URL e chave pública, nunca chave privada.
- **Depende de:** nenhum.
- **Testes:** config inválida recusa.
- **Risco:** baixo.

## Árvore de código e de teste

```text
src/                      # planejamento; código real mora em wyd-launcher/ (raiz)
├── manifest/             # manifesto JSON canônico
├── signature/            # Ed25519
├── patch/                # aplicação de patch
├── integrity/            # hash de arquivos locais
├── channels/             # lista de canais
├── news/                 # notícias
├── updater/              # auto-update
├── client-launch/        # iniciar cliente
├── http/                 # cliente HTTP
├── cache/                # cache local
└── config/               # configuração
tests/
├── unitario/            # manifest, signature, integrity
├── contrato/            # manifesto, canais
├── integracao/          # patch end-to-end local
├── sistema/             # (n/a)
├── e2e/                 # (n/a; app nativo)
└── propriedade/         # fuzz de manifesto
```

*Página de planejamento. Nenhuma unidade foi implementada.*
