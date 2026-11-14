# CLIENT-13 — Clientes alternativos e gerações visuais: o que trazer para o projeto

> **ID canônico:** `CLIENT-13`
> **ID histórico:** `CLIENT-13`
> **Estado 2026-11-14:** ⬜ · ver `roadmap-rust/index.md`.

**Tipo:** Pesquisa / decisão · **Esforço:** M · **Depende de:** `CLIENT-12` (diagnóstico do 7662) · **Bloqueia:** decisão de adotar/adaptar outro cliente

## Objetivo

Responder à “Missão Extra” do prompt master com decisão registrada: comparar gerações do cliente
7662 e clientes **oficiais/alternativos** (não de servidor privado), listar recursos visuais
presentes/ausentes em cada um e dizer o que é legal e tecnicamente viável trazer.

## Contexto

- `CLIENT-12` falsifica se a diferença percebida é ambiente (modo de vídeo/render target) ou
  conteúdo; esta issue vai além: catálogo de clientes e recursos.
- `docs/audit/19-client-visual-generations.md` já tem 5 hipóteses e comparação de assets
  (`Mesh/`, `Effect/`), mas não a matriz de recursos por cliente.
- Restrição do projeto: `client/` não é versionado; trazer binário de terceiro exige licença e
  decisão explícita, nunca cópia silenciosa.

## Passos

1. Montar matriz: cliente (versão/fonte), licença, renderer, shaders, partículas, resolução,
   animações, UI, suporte a assets do 7662.
2. Classificar cada recurso como presente nos dois, só no atual, só no antigo ou em nenhum.
3. Avaliar juridicamente/comercialmente o que pode ser distribuído e o que só pode ser
   referência técnica.
4. Propor decisão: manter 7662 + correções de ambiente, ou adaptar cliente compatível, com custo.
5. Registrar ADR se a decisão mudar a arquitetura de cliente/distribuição.

## Critérios de aceite

- [ ] Matriz de recursos por cliente, com fonte e data.
- [ ] Licenças e restrições de distribuição registradas.
- [ ] Decisão (manter/adaptar/trocar) com custo e risco.
- [ ] ADR aberta se houver mudança estrutural.

## Skills relacionadas

- `software-architecture`, `documentation-writer`, `wyd-client-connect`.
