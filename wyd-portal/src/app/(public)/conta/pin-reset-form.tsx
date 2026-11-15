"use client";

import { Mail, Send } from "lucide-react";
import { useState } from "react";

/**
 * Autoatendimento do reset da segunda senha.
 *
 * Dois passos explícitos: (1) cadastrar/atualizar o e-mail da conta, (2) pedir o link.
 * O link vale 15 minutos e só pode ser usado uma vez; a tela mostra o destino
 * mascarado devolvido pelo servidor, nunca o endereço completo nem o token.
 */
export default function PinResetForm({ initialEmail = "" }: { initialEmail?: string }) {
  const [email, setEmail] = useState(initialEmail);
  const [pending, setPending] = useState<"email" | "link" | null>(null);
  const [erro, setErro] = useState("");
  const [ok, setOk] = useState("");

  async function salvarEmail(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setErro("");
    setOk("");
    setPending("email");
    try {
      const res = await fetch("/api/panel/email", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ email }),
        cache: "no-store",
      });
      const body = (await res.json().catch(() => ({}))) as { email?: string; error?: string };
      if (res.ok && body.email) {
        setEmail(body.email);
        setOk("E-mail cadastrado. Agora você pode pedir o link.");
      } else {
        setErro(body.error ?? "Não foi possível salvar o e-mail.");
      }
    } catch {
      setErro("Serviço indisponível: tente de novo em instantes.");
    } finally {
      setPending(null);
    }
  }

  async function pedirLink() {
    setErro("");
    setOk("");
    setPending("link");
    try {
      const res = await fetch("/api/panel/pin-reset", { method: "POST", cache: "no-store" });
      const body = (await res.json().catch(() => ({}))) as { message?: string; error?: string };
      if (res.ok) {
        setOk(body.message ?? "Link enviado.");
      } else {
        setErro(body.error ?? "Não foi possível enviar o link.");
      }
    } catch {
      setErro("Serviço indisponível: tente de novo em instantes.");
    } finally {
      setPending(null);
    }
  }

  return (
    <div style={{ marginTop: 20, paddingTop: 16, borderTop: "1px solid var(--line, #2a2f3a)" }}>
      <h3 className="card__sub" style={{ marginTop: 0 }}>
        Esqueceu a segunda senha?
      </h3>
      <p className="ui-field__hint" style={{ marginTop: 0 }}>
        Cadastre o e-mail da conta e receba um link de uso único para apagar o PIN. No próximo
        acesso ao cliente você cadastra a senha numérica nova.
      </p>

      <form onSubmit={salvarEmail} className="ui-form" noValidate>
        <label className="ui-field">
          <span className="ui-field__label">E-mail da conta</span>
          <input
            className="ui-input"
            type="email"
            inputMode="email"
            autoComplete="email"
            maxLength={254}
            value={email}
            onChange={(event) => setEmail(event.target.value)}
          />
        </label>
        <div className="ui-form__actions">
          <button type="submit" className="ui-btn" disabled={pending !== null || email.trim() === ""}>
            <Mail size={15} strokeWidth={1.8} aria-hidden="true" />
            {pending === "email" ? "Salvando…" : "Salvar e-mail"}
          </button>
          <button
            type="button"
            className="ui-btn ui-btn--primary"
            onClick={pedirLink}
            disabled={pending !== null}
          >
            <Send size={15} strokeWidth={1.8} aria-hidden="true" />
            {pending === "link" ? "Enviando…" : "Enviar link de reset"}
          </button>
        </div>
      </form>

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
