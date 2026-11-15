# Autoatendimento do reset da segunda senha — prova no navegador (2026-11-15)

Fluxo percorrido num navegador real (Chromium do app, aba local) contra o portal servido em
`http://127.0.0.1:3011` (container `wyd/portal:dev` com `PIN_RESET_MAIL_ENDPOINT` apontando para
um coletor local em `127.0.0.1:8790`) e o gateway do compose. Conta de teste `qapinlink2`
(`account:1265`), criada por `accounts/create` e com o mesmo hash de PIN da `admin` antes do
reset. A `admin` não foi tocada.

As capturas de tela saíram **pretas** neste ambiente (composição do Chromium headless), então a
evidência abaixo é o **DOM renderizado** (`innerText`/estado do React), que é o que o enunciado
aceita como alternativa.

## 1. `/conta` — e-mail salvo e link enviado

Ações: login `qapinlink2` → área **Minha conta** → digitar `QA.Pin2@Example.com` em
“E-mail da conta” → **Salvar e-mail** → **Enviar link de reset**.

```
POST /api/panel/email      → 200  {"ok":true,"email":"qa.pin2@example.com"}
POST /api/panel/pin-reset  → 200
status: "Link enviado para qa*****@example.com. Ele vale por 15 minutos e só pode ser usado uma vez."
campo:  value="qa.pin2@example.com"
```

E-mail capturado pelo coletor (`tmp/mail.json`):

```json
{"from":"WYD2 <pin@teste.local>","to":["qa.pin2@example.com"],
 "subject":"Redefinir a segunda senha do WYD2",
 "text":"… Abra o link em até 15 minutos: http://127.0.0.1:3011/conta/pin-reset?token=E5Y-03vEE05dZMxc6lvicYqbbe1OV7tVf-2mIqRPZOc …"}
```

Trilha no banco: `panel.email` e `accounts.pin_reset_request` (alvo `account:1265`).

## 2. Link aberto — formulário renderizado

`GET /conta/pin-reset?token=E5Y-…` → documento 200, `title="Redefinir segunda senha · WYD2"`.

```
Ao confirmar, a segunda senha atual é apagada. No próximo acesso ao cliente do jogo você
cadastra o PIN novo no diálogo da segunda senha.

APAGAR MINHA SEGUNDA SENHA
```

## 3. Confirmação pelo botão — PIN apagado e sucesso na UI

```
antes:  "… APAGAR MINHA SEGUNDA SENHA"
clique: botão "Apagar minha segunda senha" (onClick do componente client hidratado)
depois: "… CONFIRMADO

         Segunda senha apagada. Entre no cliente do jogo com a senha da conta e cadastre o PIN
         novo no diálogo da segunda senha."
status: ["Segunda senha apagada. Entre no cliente do jogo com a senha da conta e cadastre o PIN novo no diálogo da segunda senha."]
alerta: []
```

Banco depois do clique:

```
qapinlink2 | tem_pin = f | pin_failures = 0
pin_reset_token (último) → used_at preenchido
audit_log → accounts.pin_reset_self | account:1265 | "segunda senha apagada pelo link de e-mail"
```

## 4. Mesmo token de novo — recusa legível

Recarregar o link e clicar de novo:

```
alerta: ["link inválido ou expirado"]
status: []
```

## 5. Token ausente / malformado — página não quebra

`/conta/pin-reset?token=abc` e `/conta/pin-reset` (sem token):

```
Link inválido ou incompleto

Peça um link novo em Minha conta. Se já usou este link, ele não vale de novo.
```

Sem botão de confirmação, sem erro de hidratação, sessão preservada.

## 6. Provedor não configurado — 503 honesto

Portal reiniciado **sem** `RESEND_API_KEY`/`PIN_RESET_FROM` (mesmo `SESSION_SECRET`), `/conta`
recarregada e **Enviar link de reset** clicado:

```
POST /api/panel/pin-reset → 503 (Fetch)
alerta: ["O envio de e-mail não está configurado neste servidor. Peça a redefinição pelo suporte."]
status: []
```

## Observações

- Nenhum código precisou mudar nesta rodada: os seis estados acima passaram de primeira no
  navegador (o formulário novo, a página de confirmação e as rotas do BFF já estavam corretas).
- O token em claro só apareceu no e-mail capturado; banco, log do portal e respostas HTTP carregam
  apenas o `sha256` (64 hex).
- Conta de teste, portal de teste e coletor foram removidos ao fim; `admin` segue com PIN `4321`.
