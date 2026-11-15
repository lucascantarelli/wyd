import { describe, expect, it } from "vitest";

import { hashResetToken, newResetToken, resetLink, validResetToken } from "@/server/pin-reset";

/**
 * O token do link é a credencial: ele existe só na URL entregue por e-mail e o banco
 * guarda apenas o hash. Estes testes fixam o contrato com o gateway (64 hex) e a forma
 * aceita na URL.
 */
describe("token de reset da segunda senha", () => {
  it("gera 32 bytes em base64url e passa na própria validação", () => {
    const token = newResetToken();
    expect(token).toMatch(/^[A-Za-z0-9_-]{43}$/);
    expect(validResetToken(token)).toBe(true);
  });

  it("recusa token curto, com padding ou com caractere fora do alfabeto", () => {
    expect(validResetToken("")).toBe(false);
    expect(validResetToken("a".repeat(42))).toBe(false);
    expect(validResetToken(`${"a".repeat(42)}=`)).toBe(false);
    expect(validResetToken(`${"a".repeat(42)}.`)).toBe(false);
  });

  it("hash é sha256 hex minúsculo e é estável", () => {
    const token = "a".repeat(43);
    const hash = hashResetToken(token);
    expect(hash).toMatch(/^[0-9a-f]{64}$/);
    expect(hashResetToken(token)).toBe(hash);
    expect(hashResetToken(`${token}b`)).not.toBe(hash);
  });

  it("monta o link na base do portal sem barra dupla", () => {
    const anterior = process.env.PORTAL_BASE_URL;
    process.env.PORTAL_BASE_URL = "https://portal.example.com/";
    try {
      expect(resetLink("abc")).toBe("https://portal.example.com/conta/pin-reset?token=abc");
    } finally {
      if (anterior === undefined) delete process.env.PORTAL_BASE_URL;
      else process.env.PORTAL_BASE_URL = anterior;
    }
  });
});
