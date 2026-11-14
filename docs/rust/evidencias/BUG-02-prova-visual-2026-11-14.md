# BUG-02 — tentativa de prova visual no 7662 (2026-11-14)

> **Veredito: prova visual BLOQUEADA.** O cliente abre, mas não desenha nada
> (`Error in Init Render Target Texture`); não existe tela de login para digitar senha/PIN.
> A prova de **fio** das duas recusas passou e está registrada abaixo. Nenhum critério visual
> de `BUG-02` foi marcado.

## 1. Cliente: o que foi executado

```powershell
Start-Process -FilePath 'D:\dolorestec\wyd2\client\WYD.exe' -WorkingDirectory 'D:\dolorestec\wyd2\client'
```

- Binário: SHA-256 `04c9d2a1235cba57…` (o build 7662 do roteiro).
- Janela principal: **806 × 629**, em foreground — capturada em
  [`BUG-02-cliente-7662-preto-2026-11-14.png`](BUG-02-cliente-7662-preto-2026-11-14.png).
- Conteúdo da janela: **preto uniforme**, só a barra de título “WYD CDK”. Sem server select, sem
  campo de conta/senha, sem mensagem — logo, sem como provocar ou ler uma recusa na tela.
- `client/WYD.log` no mesmo instante:

```
>>> Logging System turn on.
>>> mark
11/13 23:04:38 - VertexShader HW Accel Enabled
11/13 23:04:38 - Error in Init Render Target Texture
```

É a mesma falha de 2026-11-12 (§9 do roteiro) e de 2026-11-13. Modos de vídeo 800×600×32/16 bits
já foram testados antes sem mudar o resultado; nada do cliente foi alterado nesta tentativa.

## 2. Fio: recusas de senha e PIN no canal real

Com a stack no ar (canal 1 em `127.0.0.1:8281`) e Postgres de teste:

```bash
WYD_TEST_DSN="postgres://…/wyd" WYD_LIVE_CPSOCK="127.0.0.1:8281" \
  cargo test -p wyd-server --test sistema_autenticacao -- --nocapture
```

Resultado: `1 passed; 0 failed` (20,71 s). O teste abre socket real e confere, além do opcode,
o **corpo** do `_MSG_MessagePanel` que o 7662 desenha.

Log do canal no intervalo (conta descartável do teste, sem credencial no texto):

```
level=warn event=login result=refused account=aa91493748900 reason=senha de login não confere
level=warn event=handshake result=rejected reason=autenticação: senha de login não confere
level=info event=login result=password_ok account=aa91493748900 version=12000 channels=2
level=warn event=pin result=refused account=aa91493748900 reason=senha numérica não confere
level=warn event=handshake result=rejected reason=senha numérica: senha numérica não confere
level=info event=login result=password_ok account=aa91493748900 version=12000 channels=2
level=info event=pin result=ok account=aa91493748900 mode=Verified change=0
level=info event=handshake result=accepted version=12000 channels=2 kw=11 session=387
```

O que o teste prova, em ordem: senha errada recebe `NewAccountFail` + `MessagePanel`
(`Senha ou conta incorreta`) e **não** recebe `CNFAccountLogin`; PIN errado recebe
`AccountSecureFail` + `MessagePanel` e não abre sessão; PIN certo é aceito e o login segue.

## 3. O que falta para a prova visual

| Item | Estado |
|---|---|
| Print da recusa de senha no 7662 | ⬜ bloqueado: janela preta |
| Print da recusa de PIN no 7662 | ⬜ bloqueado: janela preta |
| Log do canal para as duas recusas | ✅ (acima) |
| Frame de recusa conferido (`NewAccountFail`/`AccountSecureFail` + painel) | ✅ (teste) |

Pré-requisito para destravar: D3D9 com render target funcional (VM com driver de vídeo real ou
máquina de teste), como pede o roteiro §2.1. Sem isso, qualquer “print de recusa” seria fabricado.

## 4. Artefatos

- Versionado: este documento + o print da janela preta.
- Local (ignorado pelo git): `tmp/prova-visual-7662/` (`autenticacao.log`, `canal.log`,
  `wyd-stdout.log`, `A0-cliente.png`, `janela.ps1`).
