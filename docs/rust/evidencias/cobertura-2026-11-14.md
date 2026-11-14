# Cobertura — baseline 2026-11-14

Medição feita no merge da `develop` (`41aab90`, depois de `make check` verde), como ponto de
partida da `QA-15`. **Nada aqui é meta atingida** — são os números reais antes do programa de
testes.

## Comandos

```bash
cargo llvm-cov --workspace --all-targets --summary-only   # Rust
cd wyd-portal && npm run test:cobertura                   # portal (vitest + v8)
```

## Rust — linhas por crate

| Crate | Linhas | Cobertura de linha |
|---|---:|---:|
| `wyd-core` | — | 91,1% |
| `wyd-sim` | 1.722 | 96,2% |
| `wyd-net` | 439 | 96,1% |
| `wyd-protocol` | 1.909 | 74,8% |
| `wyd-db` | — | 10,5% |
| `wyd-server` | 8.497 | 51,9% |
| `wyd-admin` | 6.705 | 54,9% |
| `wyd-bots` | — | 24,4% |
| `wyd-launcher` | — | 72,1% |
| `launcher-ui` | 162 | 23,5% |
| **Total workspace** | **38.598** | **58,1%** (regiões 62,2%; funções 58,7%) |

## Portal

| Métrica | Valor |
|---|---:|
| Statements | 52,7% |
| Branches | 44,7% |
| Functions | 44,7% |
| Lines | 53,8% |
| Testes | 154 em 29 arquivos, verdes |

## Leitura

- Só `wyd-sim` e `wyd-net` já cumprem o piso de 95% unitário; `wyd-core` (91,1%) está perto.
- `wyd-server` (51,9%) concentra a maior massa de linhas descobertas e é o alvo de maior risco.
- `wyd-db` (10,5%), `wyd-bots` (24,4%) e `launcher-ui` (23,5%) não têm suíte de unidade: a
  cobertura vem só de integração ou de nada.
- Portal em 53,8% de linha: páginas e handlers sem teste de componente/unidade.

## O que falta para o gate da `QA-15`

1. Piso por crate em script (95% unitário, 80% integração, 80% global).
2. Fechar `wyd-server` e `wyd-admin` (maior ganho absoluto) antes de perseguir décimos.
3. Suíte própria para `wyd-db`/`wyd-bots` (hoje quase sem alvo de teste).
4. Portal: cobrir `src/app/**` e os Route Handlers que não aparecem no relatório.
