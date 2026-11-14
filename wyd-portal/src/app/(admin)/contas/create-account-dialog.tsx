"use client";

import { UserPlus } from "lucide-react";
import { useRouter } from "next/navigation";
import { useState } from "react";

import { Dialog } from "@/components/Dialog";
import { Field } from "@/components/Field";
import { useToast } from "@/components/Toast";
import { ROLES, roleLabel, type Role } from "@/lib/role";
import { erroSenhaConta, senhaJogoBytes } from "@/lib/senha";

import { createAccountAction } from "./actions";

/**
 * Criação de conta pelo painel.
 *
 * A senha nasce junto com a conta: é o único cadastro do jogador e a mesma
 * credencial serve ao portal e ao cliente (8 a 12 **bytes**). A senha numérica
 * (PIN) continua sendo criada no primeiro acesso do cliente.
 */
export function CreateAccountDialog({ canCreateStaff }: { canCreateStaff: boolean }) {
  const router = useRouter();
  const toast = useToast();
  const [open, setOpen] = useState(false);
  const [pending, setPending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [login, setLogin] = useState("");
  const [password, setPassword] = useState("");
  const [role, setRole] = useState<Role>("player");

  async function submit() {
    if (login.trim() === "") {
      setError("informe o login");
      return;
    }
    const senhaInvalida = erroSenhaConta(password, role);
    if (senhaInvalida) {
      setError(senhaInvalida);
      return;
    }
    setPending(true);
    const result = await createAccountAction({ login, password, role });
    setPending(false);
    if (!result.ok) {
      setError(result.error);
      return;
    }
    toast(result.message, "ok");
    setOpen(false);
    setError(null);
    setLogin("");
    setPassword("");
    setRole("player");
    router.refresh();
  }

  return (
    <>
      <button className="ui-btn ui-btn--primary" onClick={() => setOpen(true)}>
        <UserPlus size={15} strokeWidth={1.8} aria-hidden="true" />
        Criar conta
      </button>

      <Dialog
        open={open}
        title="Criar conta"
        description="A senha é a mesma do portal e do jogo: o jogador entra no cliente com ela. O PIN é criado no primeiro login do personagem."
        onClose={() => setOpen(false)}
        footer={
          <>
            <button
              className="ui-btn ui-btn--ghost"
              onClick={() => setOpen(false)}
              disabled={pending}
            >
              Cancelar
            </button>
            <button className="ui-btn ui-btn--primary" onClick={submit} disabled={pending}>
              {pending ? "Criando…" : "Criar conta"}
            </button>
          </>
        }
      >
        <div className="ui-form" style={{ maxWidth: "none" }}>
          <Field label="Login" hint="4 a 12 letras ou números, sem espaço">
            <input
              className="ui-input"
              value={login}
              maxLength={12}
              autoComplete="off"
              autoFocus
              onChange={(event) => setLogin(event.target.value)}
            />
          </Field>

          <Field
            label="Senha"
            hint={
              role === "player"
                ? `8 a 12 bytes — a mesma senha do portal e do jogo (${senhaJogoBytes(password)}/12)`
                : "mínimo 8 bytes — staff não digita senha no cliente"
            }
          >
            <input
              className="ui-input"
              type="password"
              value={password}
              autoComplete="new-password"
              onChange={(event) => setPassword(event.target.value)}
            />
          </Field>

          <Field
            label="Papel"
            hint={canCreateStaff ? "administrador pode criar qualquer papel" : "você só cria jogador"}
          >
            <select
              className="ui-select"
              value={role}
              onChange={(event) => setRole(event.target.value as Role)}
              disabled={!canCreateStaff}
            >
              {ROLES.filter((option) => canCreateStaff || option === "player").map((option) => (
                <option key={option} value={option}>
                  {roleLabel(option)}
                </option>
              ))}
            </select>
          </Field>

          {error ? (
            <p className="ui-field__error" role="alert">
              {error}
            </p>
          ) : null}
        </div>
      </Dialog>
    </>
  );
}
