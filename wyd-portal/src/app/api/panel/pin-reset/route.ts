import { TEXTO } from "@/lib/i18n";
import { AdminError, toResponse } from "@/server/errors";
import { startPinReset } from "@/server/panel";
import {
  hashResetToken,
  mailConfigured,
  newResetToken,
  resetLink,
  sendPinResetMail,
} from "@/server/pin-reset";
import { getSession } from "@/server/session";

export const dynamic = "force-dynamic";

const NO_STORE = { "cache-control": "no-store" } as const;

/**
 * Envia o link de reset da segunda senha para o e-mail da própria conta.
 *
 * Sem sessão é 401; sem e-mail cadastrado o gateway responde 422; sem provedor de
 * e-mail configurado é 503 (nunca "enviado"). O token só existe no link: o corpo
 * da resposta não o repete e o log não o vê.
 */
export async function POST() {
  const session = await getSession();
  if (!session) {
    return Response.json({ error: TEXTO.pin.sessaoExpirada }, { status: 401, headers: NO_STORE });
  }

  if (!mailConfigured()) {
    return Response.json(
      {
        error:
          "O envio de e-mail não está configurado neste servidor. Peça a redefinição pelo suporte.",
      },
      { status: 503, headers: NO_STORE },
    );
  }

  const token = newResetToken();
  let destino: string;
  try {
    destino = await startPinReset(hashResetToken(token));
  } catch (err) {
    if (err instanceof AdminError && err.status === 422) {
      return Response.json(
        { error: "Cadastre um e-mail na conta antes de pedir o link." },
        { status: 422, headers: NO_STORE },
      );
    }
    return toResponse(err);
  }

  try {
    await sendPinResetMail({ to: destino, link: resetLink(token) });
  } catch {
    return Response.json(
      { error: "Não foi possível enviar o e-mail agora. Tente de novo em instantes." },
      { status: 502, headers: NO_STORE },
    );
  }

  // Destino mascarado: confirma o envio sem imprimir o endereço inteiro na tela.
  const [usuario, dominio] = destino.split("@");
  const mascarado = `${usuario.slice(0, 2)}${"*".repeat(Math.max(usuario.length - 2, 1))}@${dominio}`;
  return Response.json(
    {
      ok: true,
      message: `Link enviado para ${mascarado}. Ele vale por 15 minutos e só pode ser usado uma vez.`,
    },
    { headers: NO_STORE },
  );
}
