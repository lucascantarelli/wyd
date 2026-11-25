> Pergunta: "quais requisitos o `wyd-protocol` precisa cumprir na `1.0.0` e como saber que cada um está atendido?"
> Público: Desenvolvedor novo, produto e QA
> Dono: produto
> Base: `wyd-protocol-1.0.0.md` §1.1, §1.4; `wyd-portal/docs/specs/requisitos.md` (molde, não conteúdo)
> Revisado em: 2026-11-25

**Estado:** proposto (2026-11-25). Requisitos derivados do plano diretor; números de byte exigem golden (`LPRT-10`).

Requisito sem critério de aceitação é intenção. Numeração `RF-PRT-nnn` (funcional) e `RNF-PRT-nnn` (não funcional); não se renumera o que já existe. Todo requisito de layout carrega `V` (golden/captura) ou `H` com `LPRT-nn`.

## Requisitos funcionais

### RF-PRT-001 — Decodificar o HEADER de 12 bytes
- **Entrada:** buffer com pelo menos 12 bytes.
- **Saída:** `Header` com campos tipados.
- **Aceitação:** offsets conferem com golden `G-header-*`; campo fora de faixa é recusado; leitura explícita, sem `unsafe`.
- **Contrato:** `LPRT-05`, `LPRT-07`.

### RF-PRT-002 — Enquadrar pacote por `Size`
- **Entrada:** buffer com bytes concatenados.
- **Saída:** frame completo ou estado de buffer (parcial/colado/lixo).
- **Aceitação:** pacote parcial não é erro fatal; `Size = 8192` aceito, acima recusado com erro tipado; diagrama de estados do buffer.
- **Contrato:** `LPRT-07`.

### RF-PRT-003 — Transformar payload por ofuscação e checksum
- **Entrada:** bytes ofuscados.
- **Saída:** payload em claro ou erro tipado.
- **Aceitação:** checksum divergente recusado sem modo tolerante; exemplo numérico de 8 bits; semente recebida do chamador (`LPRT-03`).
- **Contrato:** `LPRT-02`, `LPRT-03`, `LPRT-17`.

### RF-PRT-004 — Mapear `Type` para mensagem tipada
- **Entrada:** `Type` do header.
- **Saída:** mensagem tipada ou `UnknownType`.
- **Aceitação:** `Type` fora do catálogo devolve `UnknownType`, não queda; combinação inválida de flags é erro tipado.
- **Contrato:** `LPRT-08`, `LPRT-11`.

### RF-PRT-005 — Entregar payload tipado e validado
- **Entrada:** bytes do payload.
- **Saída:** struct tipada por família.
- **Aceitação:** faixa violada recusada; bytes crus não saem do crate (`LPRT-18`); campo sem proveniência reprova.
- **Contrato:** `LPRT-04`, `LPRT-06`, `LPRT-18`.

### RF-PRT-006 — Gerar catálogo de opcodes reprodutível
- **Entrada:** tabela versionada.
- **Saída:** `Opcode` enum + `KEY_TABLE`.
- **Aceitação:** saída determinística; diff no CI; total medido do gerador, nunca herdado do legado.
- **Contrato:** `LPRT-01`, `LPRT-16`, `LPRT-20`.

## Requisitos não funcionais

| ID | Requisito | Critério verificável |
|---|---|---|
| RNF-PRT-001 | Determinismo do codec | mesmo input → mesmo output, em todas as plataformas |
| RNF-PRT-002 | Sem `unsafe` na leitura do fio | varredura de código sem `unsafe` |
| RNF-PRT-003 | Sem panic em decode | fuzz não panica em nenhum input até o limite |
| RNF-PRT-004 | Tempo de decode dentro do orçamento | medição por mensagem, P99 declarado |
| RNF-PRT-005 | Cobertura mínima de 85% por capacidade | gate de cobertura com código gerado fora da métrica de linha |
| RNF-PRT-006 | Robustez contra input malicioso | limites de tamanho e profundidade aplicados |

*Página de planejamento. Nenhum requisito foi implementado.*