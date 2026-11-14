import { describe, expect, it } from "vitest";

import { erroSenhaConta, erroSenhaJogo, senhaJogoBytes } from "@/lib/senha";

/**
 * O limite é do fio (`char[12]`), não da tela: medir caractere em vez de byte
 * aceitaria "ççççççç" (14 bytes) e o cliente truncaria a senha no login.
 */
describe("senha de jogador", () => {
  it("mede bytes UTF-8, não caracteres", () => {
    // a(1) + ç(2) + ã(2) + o(1) = 6 bytes em UTF-8.
    expect(senhaJogoBytes("ação")).toBe(6);
    expect(senhaJogoBytes("senha123")).toBe(8);
  });

  it("aceita de 8 a 12 bytes", () => {
    expect(erroSenhaJogo("senha123")).toBeNull();
    expect(erroSenhaJogo("abcdefghijkl")).toBeNull();
  });

  it("recusa abaixo de 8 e acima de 12 bytes", () => {
    expect(erroSenhaJogo("1234567")).toMatch(/pelo menos 8 bytes/);
    expect(erroSenhaJogo("abcdefghijklm")).toMatch(/no máximo 12 bytes/);
    expect(erroSenhaJogo("açãoaçãoação")).toMatch(/no máximo 12 bytes/);
  });
});

describe("senha no cadastro do portal", () => {
  it("staff não tem o teto do cliente", () => {
    expect(erroSenhaConta("abcdefghijklm", "admin")).toBeNull();
    expect(erroSenhaConta("curta", "moderator")).toMatch(/pelo menos 8 bytes/);
  });

  it("jogador continua limitado a 12 bytes", () => {
    expect(erroSenhaConta("abcdefghijklm", "player")).toMatch(/no máximo 12 bytes/);
    expect(erroSenhaConta("abcdefghijkl", "player")).toBeNull();
  });
});
