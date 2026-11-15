"use client";

import { KeyRound } from "lucide-react";
import { useState } from "react";

/**
 * Confirma o link recebido por e-mail.
 *
 * O token vem da URL da página e vai no corpo do POST (nunca numa URL de log). O
 * servidor só apaga a segunda senha; o PIN novo é cadastrado no cliente do jogo.
 */
export default function ConfirmPinResetForm({ token }: { token: string }) {
  const [pending, setPending] = useState(false);
  const [erro, setErro] = useState("");
  const [ok, setOk] = useState("");

  async function confirmar() {
    setErro("");
    setOk("");
    setPending(true);
    try {
      const res = await fetch("/api/panel/pin-reset/confirmar", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ token }),
        cache: "no-store",
      });
      const body = (await res.json().catch(() => ({}))) as { message?: string; error?: string };
      if (res.ok) {
        setOk(body.message ?? "Segunda senha apagada.");
      } else {
        setErro(body.error ?? "Não foi possível confirmar o link.");
      }
    } catch {
      setErro("Serviço indisponível: tente de novo em instantes.");
    } finally {
      setPending(false);
    }
  }

  return (
    <div className="ui-form">
      <div className="ui-form__actions">
        <button
          type="button"
          className="ui-btn ui-btn--primary"
          onClick={confirmar}
          disabled={pending || ok !== ""}
        >
          <KeyRound size={15} strokeWidth={1.8} aria-hidden="true" />
          {pending ? "Confirmando…" : ok ? "Confirmado" : "Apagar minha segunda senha"}
        </button>
      </div>
      {erro ? (
        <p role="alert" className="ui-field__error">
          {erro}
        </p>
      ) : null}
      {ok ? (
        <p role="status" className="text-ok">
          {ok}
        </p>
      ) : null}
    </div>
  );
}
