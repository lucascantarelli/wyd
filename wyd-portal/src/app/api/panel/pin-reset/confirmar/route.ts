import { z } from "zod";

import { TEXTO } from "@/lib/i18n";
import { AdminError, toResponse } from "@/server/errors";
import { confirmPinReset } from "@/server/panel";
import { hashResetToken, validResetToken } from "@/server/pin-reset";
import { getSession } from "@/server/session";

export const dynamic = "force-dynamic";

const NO_STORE = { "cache-control": "no-store" } as const;

const Body = z.object({ token: z.string().min(1).max(128) });

/**
 * Confirma o link e apaga a segunda senha (uso único).
 *
 * O token vai só no corpo, nunca em URL de log. Expirado, já usado ou desconhecido
 * respondem a mesma recusa. Depois disso o jogador entra no cliente e cadastra o
 * PIN novo — o cliente 7662 não tem tela de troca.
 */
export async function POST(req: Request) {
  const session = await getSession();
  if (!session) {
    return Response.json({ error: TEXTO.pin.sessaoExpirada }, { status: 401, headers: NO_STORE });
  }

  let parsed: z.infer<typeof Body>;
  try {
    parsed = Body.parse(await req.json());
  } catch {
    return Response.json({ error: "link inválido ou expirado" }, { status: 422, headers: NO_STORE });
  }
  if (!validResetToken(parsed.token)) {
    return Response.json({ error: "link inválido ou expirado" }, { status: 422, headers: NO_STORE });
  }

  try {
    await confirmPinReset(hashResetToken(parsed.token));
  } catch (err) {
    if (err instanceof AdminError && err.status === 422) {
      return Response.json(
        { error: "link inválido ou expirado" },
        { status: 422, headers: NO_STORE },
      );
    }
    return toResponse(err);
  }

  return Response.json(
    {
      ok: true,
      message:
        "Segunda senha apagada. Entre no cliente do jogo com a senha da conta e cadastre o PIN novo no diálogo da segunda senha.",
    },
    { headers: NO_STORE },
  );
}
