import { z } from "zod";

import { TEXTO } from "@/lib/i18n";
import { AdminError, toResponse } from "@/server/errors";
import { changeNumericPin, getPanelAccount } from "@/server/panel";
import { getSession } from "@/server/session";

export const dynamic = "force-dynamic";

const NO_STORE = { "cache-control": "no-store" } as const;

/** Mesma regra de PIN do servidor (`service::pin::valid_pin`). */
const PIN = z.string().regex(/^\d{4,6}$/, TEXTO.pin.regras);

/**
 * Campo de senha do `0x0FDB`, em **bytes**: o cliente não envia além disso.
 *
 * Contar caracteres aceitaria 7 acentos (14 bytes) e o gateway mandaria ao
 * `AccountLogin` uma senha truncada — o jogador veria "PIN atual inválido" sem
 * nunca ter errado o PIN.
 */
const GAME_PASSWORD_MAX_BYTES = 12;

/**
 * Corpo aceito: o `login` vem da sessão, não deste JSON.
 *
 * `password` é exigida porque o `0x0FDE` só existe depois do `AccountLogin` no
 * servidor de jogo — o gateway usa a senha para abrir a sessão e executar as duas
 * etapas. Ela não é guardada em lugar nenhum.
 */
const Body = z.object({
  // O limite de 12 bytes é conferido antes do parse, em bytes UTF-8.
  password: z.string().min(1),
  current_pin: PIN,
  new_pin: PIN,
});

/**
 * Troca a senha numérica do jogador logado no portal.
 *
 * O `account_id` sai do cookie assinado (`getSession`); o login vem do backend
 * (`panel`), nunca do browser. A troca em si roda no gateway contra o servidor de
 * jogo, no mesmo fluxo de duas etapas do cliente.
 */
export async function POST(req: Request) {
  const session = await getSession();
  if (!session) {
    return Response.json({ error: TEXTO.pin.sessaoExpirada }, { status: 401, headers: NO_STORE });
  }

  let raw: unknown;
  try {
    raw = await req.json();
  } catch {
    return Response.json({ error: TEXTO.pin.regras }, { status: 422, headers: NO_STORE });
  }

  const senha = (raw as { password?: unknown } | null)?.password;
  if (
    typeof senha === "string" &&
    new TextEncoder().encode(senha).length > GAME_PASSWORD_MAX_BYTES
  ) {
    // 413 e não 422: a tela já usa 422 para "PIN atual não confere" e não pode
    // confundir senha longa com PIN errado.
    return Response.json({ error: TEXTO.pin.senhaLonga }, { status: 413, headers: NO_STORE });
  }

  let parsed: z.infer<typeof Body>;
  try {
    parsed = Body.parse(raw);
  } catch {
    return Response.json({ error: TEXTO.pin.regras }, { status: 422, headers: NO_STORE });
  }

  if (parsed.current_pin === parsed.new_pin) {
    return Response.json(
      { error: TEXTO.pin.igualAoAtual },
      { status: 422, headers: NO_STORE },
    );
  }

  try {
    // O login vem da conta da sessão — o browser não escolhe de quem é o PIN.
    const conta = await getPanelAccount();
    await changeNumericPin(conta.login, {
      password: parsed.password,
      currentPin: parsed.current_pin,
      newPin: parsed.new_pin,
    });
  } catch (err) {
    if (err instanceof AdminError && err.status === 400) {
      return Response.json({ error: TEXTO.pin.atualInvalido }, { status: 400, headers: NO_STORE });
    }
    return toResponse(err);
  }

  return Response.json({ ok: true, changed: true }, { headers: NO_STORE });
}
