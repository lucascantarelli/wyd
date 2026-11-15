/**
 * Autoatendimento do reset da segunda senha — lado do portal.
 *
 * O portal gera o token (aleatório, 256 bits), manda o link por e-mail e entrega ao
 * gateway só o **sha256** dele. O token em claro existe apenas na URL que o jogador
 * recebe: não entra em log, banco nem resposta de erro.
 *
 * Envio: `RESEND_API_KEY` + `PIN_RESET_FROM` (remetente verificado no provedor). Sem as
 * duas variáveis a rota responde 503 — o portal nunca "finge" que enviou.
 */
import { createHash, randomBytes } from "node:crypto";

/**
 * Provedor de e-mail transacional (uma chave, um POST JSON).
 *
 * `PIN_RESET_MAIL_ENDPOINT` permite apontar para um coletor local em teste/desenvolvimento;
 * em produção fica o padrão do provedor.
 */
const MAIL_ENDPOINT = process.env.PIN_RESET_MAIL_ENDPOINT ?? "https://api.resend.com/emails";
/** Mesmo TTL do gateway (`api::pin_reset::TOKEN_TTL_SECONDS`). */
export const TOKEN_TTL_MINUTES = 15;

/** Token do link: base64url de 32 bytes (43 caracteres, sem padding). */
export function newResetToken(): string {
  return randomBytes(32).toString("base64url");
}

/** `sha256` hex — o único formato que o gateway aceita guardar. */
export function hashResetToken(token: string): string {
  return createHash("sha256").update(token, "utf8").digest("hex");
}

/** Forma mínima do token recebido pela URL antes de qualquer hash. */
export function validResetToken(token: string): boolean {
  return /^[A-Za-z0-9_-]{43}$/.test(token);
}

/** Base pública do portal para montar o link (nunca derivada do Host da requisição). */
export function portalBaseUrl(): string {
  return (process.env.PORTAL_BASE_URL ?? "http://127.0.0.1:3000").replace(/\/+$/, "");
}

/** Link de confirmação (o token vai na query; a página não o loga). */
export function resetLink(token: string): string {
  return `${portalBaseUrl()}/conta/pin-reset?token=${encodeURIComponent(token)}`;
}

/** O envio está configurado? Sem isso o link não sairia e o pedido é recusado. */
export function mailConfigured(): boolean {
  return Boolean(process.env.RESEND_API_KEY && process.env.PIN_RESET_FROM);
}

/**
 * Envia o link. Falha de provedor vira erro de indisponibilidade, nunca "enviado":
 * um token emitido sem entrega confundiria o jogador e ficaria válido por 15 min.
 */
export async function sendPinResetMail(input: { to: string; link: string }): Promise<void> {
  if (!mailConfigured()) {
    throw new Error("envio de e-mail não configurado");
  }
  const resposta = await fetch(MAIL_ENDPOINT, {
    method: "POST",
    headers: {
      authorization: `Bearer ${process.env.RESEND_API_KEY}`,
      "content-type": "application/json",
    },
    body: JSON.stringify({
      from: process.env.PIN_RESET_FROM,
      to: [input.to],
      subject: "Redefinir a segunda senha do WYD2",
      text: [
        "Alguém pediu para redefinir a segunda senha (PIN) desta conta no WYD2.",
        "",
        `Abra o link em até ${TOKEN_TTL_MINUTES} minutos: ${input.link}`,
        "",
        "Depois de confirmar, entre no cliente do jogo: o PIN é cadastrado de novo no",
        "diálogo da segunda senha.",
        "",
        "Se não foi você, ignore esta mensagem: o link expira sozinho e sua senha atual continua valendo.",
      ].join("\n"),
    }),
    signal: AbortSignal.timeout(10_000),
    cache: "no-store",
  });
  if (!resposta.ok) {
    throw new Error(`provedor de e-mail recusou o envio (${resposta.status})`);
  }
}
