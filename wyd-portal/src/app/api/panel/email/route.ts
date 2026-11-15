import { z } from "zod";

import { TEXTO } from "@/lib/i18n";
import { AdminError, toResponse } from "@/server/errors";
import { setAccountEmail } from "@/server/panel";
import { getSession } from "@/server/session";

export const dynamic = "force-dynamic";

const NO_STORE = { "cache-control": "no-store" } as const;

/** Mesmo teto da constraint `accounts_email_formato` e da validação do gateway. */
const Body = z.object({
  email: z.string().trim().min(3).max(254),
});

/**
 * Cadastra o e-mail de recuperação da própria conta.
 *
 * O endereço é o canal de prova do reset da segunda senha. O `account_id` sai da
 * sessão assinada; o gateway revalida a forma e audita com o autor.
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
    return Response.json({ error: "informe um e-mail válido" }, { status: 422, headers: NO_STORE });
  }

  try {
    const email = await setAccountEmail(parsed.email);
    return Response.json({ ok: true, email }, { headers: NO_STORE });
  } catch (err) {
    if (err instanceof AdminError && err.status === 422) {
      return Response.json({ error: "informe um e-mail válido" }, { status: 422, headers: NO_STORE });
    }
    return toResponse(err);
  }
}
