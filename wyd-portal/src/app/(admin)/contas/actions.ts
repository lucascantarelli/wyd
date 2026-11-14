"use server";

/**
 * Criação de conta (Server Action).
 *
 * O `moderator_id` nunca vem do formulário: `adminCall` injeta o da sessão. O
 * papel é validado aqui e novamente no gateway — moderador só cria jogador.
 *
 * A senha é definida **aqui**, no cadastro: é o único momento do jogador. Ela
 * vale no portal e no cliente (mesmo `password_hash`), então cabe nos 12 bytes
 * do login CPSock — o gateway revalida antes de gravar.
 */
import { revalidatePath } from "next/cache";
import { z } from "zod";

import { ROLES, type Role } from "@/lib/role";
import { erroSenhaConta } from "@/lib/senha";

import { createAccount } from "@/server/accounts";
import { AdminError } from "@/server/errors";

export type CreateAccountResult = { ok: true; message: string } | { ok: false; error: string };

const schema = z.object({
  login: z
    .string()
    .trim()
    .min(4, "o login precisa de pelo menos 4 caracteres")
    .max(12, "o login aceita no máximo 12 caracteres")
    .regex(/^[a-z0-9]+$/i, "use apenas letras e números"),
  password: z.string().min(1, "informe a senha"),
  role: z.enum(["player", "moderator", "admin"]),
});

export async function createAccountAction(input: {
  login: string;
  password: string;
  role: Role;
}): Promise<CreateAccountResult> {
  const parsed = schema.safeParse(input);
  if (!parsed.success) {
    return { ok: false, error: parsed.error.issues[0]?.message ?? "dados inválidos" };
  }
  if (!ROLES.includes(parsed.data.role)) {
    return { ok: false, error: "papel inválido" };
  }
  const senhaInvalida = erroSenhaConta(parsed.data.password, parsed.data.role);
  if (senhaInvalida) {
    return { ok: false, error: senhaInvalida };
  }

  try {
    const account = await createAccount({
      login: parsed.data.login.toLowerCase(),
      password: parsed.data.password,
      role: parsed.data.role,
    });
    revalidatePath("/contas");
    return { ok: true, message: `conta ${account.login} criada` };
  } catch (error) {
    if (error instanceof AdminError) {
      return { ok: false, error: error.message };
    }
    return { ok: false, error: "serviço de administração indisponível" };
  }
}
