/**
 * Regra da senha de jogador — a MESMA do portal e do cliente.
 *
 * O login CPSock carrega a senha em `char account_password[12]`, então o limite
 * é medido em **bytes** UTF-8 (acento conta dois), não em caracteres. O portal
 * grava o hash Argon2id que o cliente também verifica: senha fora do limite
 * criaria conta que o jogador não consegue usar no jogo.
 */

/** Mínimo aceito (mesma política de `MIN_PASSWORD_LEN` no gateway). */
export const SENHA_JOGO_MIN_BYTES = 8;

/** Teto do campo `account_password[12]` do `_MSG_AccountLogin`. */
export const SENHA_JOGO_MAX_BYTES = 12;

/** Bytes UTF-8 da senha (o que o servidor mede). */
export function senhaJogoBytes(senha: string): number {
  return new TextEncoder().encode(senha).length;
}

/** Papéis do portal (o teto do fio só vale para quem joga). */
export type PapelConta = "player" | "moderator" | "admin";

/** Erro da senha, ou `null` se ela cabe no protocolo. */
export function erroSenhaJogo(senha: string): string | null {
  const bytes = senhaJogoBytes(senha);
  if (bytes < SENHA_JOGO_MIN_BYTES) {
    return `a senha precisa de pelo menos ${SENHA_JOGO_MIN_BYTES} bytes`;
  }
  if (bytes > SENHA_JOGO_MAX_BYTES) {
    return `a senha do jogo aceita no máximo ${SENHA_JOGO_MAX_BYTES} bytes (acento conta dois)`;
  }
  return null;
}

/**
 * Erro da senha no cadastro do portal: jogador cabe nos 12 bytes do login CPSock;
 * staff não digita senha no cliente, então só precisa do mínimo.
 */
export function erroSenhaConta(senha: string, papel: PapelConta): string | null {
  if (papel === "player") return erroSenhaJogo(senha);
  return senhaJogoBytes(senha) < SENHA_JOGO_MIN_BYTES
    ? `a senha precisa de pelo menos ${SENHA_JOGO_MIN_BYTES} bytes`
    : null;
}
