import { beforeEach, describe, expect, it, vi } from "vitest";

import { TEXTO } from "@/lib/i18n";
import { AdminError } from "@/server/errors";
import type { Session } from "@/server/session";

/**
 * Troca de PIN: a rota só aceita corpo válido e **delega ao gateway**.
 *
 * O que o teste prova: sem sessão não há troca; PIN fora de 4–6 dígitos e PIN
 * novo igual ao atual param antes do backend; a recusa de PIN atual do servidor de
 * jogo (400) vira mensagem da tela; e no sucesso a rota usa o `accountId` do
 * cookie, nunca um id do corpo.
 */

const state: {
  session: Session | null;
  calls: { login: string; body: unknown }[];
  fail: unknown;
} = { session: null, calls: [], fail: null };

vi.mock("@/server/session", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@/server/session")>();
  return { ...actual, getSession: async () => state.session };
});

vi.mock("@/server/panel", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@/server/panel")>();
  return {
    ...actual,
    getPanelAccount: async () => ({ id: 7, login: "jogador", role: "player" }),
    changeNumericPin: async (login: string, body: unknown) => {
      state.calls.push({ login, body });
      if (state.fail) throw state.fail;
      return { changed: true };
    },
  };
});

const { POST } = await import("@/app/api/panel/pin/route");

function req(body: unknown): Request {
  return new Request("http://127.0.0.1:3000/api/panel/pin", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
}

const valido = { password: "SenhaForte1", current_pin: "1357", new_pin: "8642" };

describe("POST /api/panel/pin", () => {
  beforeEach(() => {
    state.session = null;
    state.calls = [];
    state.fail = null;
  });

  it("sem sessão devolve 401 e não chama o gateway", async () => {
    const res = await POST(req(valido));

    expect(res.status).toBe(401);
    expect(state.calls).toHaveLength(0);
  });

  it("PIN fora de 4–6 dígitos devolve 422 antes do backend", async () => {
    state.session = { accountId: 7, role: "player" };

    const res = await POST(req({ ...valido, new_pin: "12" }));

    expect(res.status).toBe(422);
    expect(state.calls).toHaveLength(0);
  });

  it("senha acima de 12 bytes devolve 413 antes do backend", async () => {
    state.session = { accountId: 7, role: "player" };

    const res = await POST(req({ ...valido, password: "a".repeat(13) }));

    expect(res.status).toBe(413);
    expect(await res.json()).toMatchObject({ error: TEXTO.pin.senhaLonga });
    expect(state.calls).toHaveLength(0);
  });

  it("conta bytes, não caracteres: 7 acentos passam de 12 bytes", async () => {
    state.session = { accountId: 7, role: "player" };
    expect("á".repeat(7)).toHaveLength(7);

    const res = await POST(req({ ...valido, password: "á".repeat(7) }));

    expect(res.status).toBe(413);
    expect(await res.json()).toMatchObject({ error: TEXTO.pin.senhaLonga });
    expect(state.calls).toHaveLength(0);
  });

  it("senha de 12 bytes (6 acentos) chega intacta ao gateway", async () => {
    state.session = { accountId: 7, role: "player" };
    const senha = "á".repeat(6);
    expect(new TextEncoder().encode(senha)).toHaveLength(12);

    const res = await POST(req({ ...valido, password: senha }));

    expect(res.status).toBe(200);
    expect(state.calls).toEqual([
      { login: "jogador", body: { password: senha, currentPin: "1357", newPin: "8642" } },
    ]);
  });

  it("PIN novo igual ao atual devolve 422", async () => {
    state.session = { accountId: 7, role: "player" };

    const res = await POST(req({ ...valido, new_pin: "1357" }));

    expect(res.status).toBe(422);
    expect(state.calls).toHaveLength(0);
  });

  it("PIN atual recusado pelo servidor de jogo vira mensagem na tela", async () => {
    state.session = { accountId: 7, role: "player" };
    state.fail = new AdminError(400, "PIN atual inválido ou conta bloqueada por tentativas");

    const res = await POST(req(valido));

    expect(res.status).toBe(400);
    expect(await res.json()).toMatchObject({ error: TEXTO.pin.atualInvalido });
    expect(state.calls).toHaveLength(1);
  });

  it("sucesso troca pelo login da conta da sessão", async () => {
    state.session = { accountId: 7, role: "player" };

    const res = await POST(req({ ...valido, login: "outro", account_id: 99 }));

    expect(res.status).toBe(200);
    expect(await res.json()).toEqual({ ok: true, changed: true });
    expect(state.calls).toEqual([
      {
        login: "jogador",
        body: { password: "SenhaForte1", currentPin: "1357", newPin: "8642" },
      },
    ]);
  });
});
