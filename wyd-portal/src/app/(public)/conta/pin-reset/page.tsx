import { KeyRound } from "lucide-react";
import Link from "next/link";
import { redirect } from "next/navigation";

import { SiteHeader } from "@/components/SiteHeader";
import { getSession } from "@/server/session";
import { validResetToken } from "@/server/pin-reset";

import ConfirmPinResetForm from "./confirm-form";

export const dynamic = "force-dynamic";

export const metadata = { title: "Redefinir segunda senha · WYD2" };

/**
 * Página do link enviado por e-mail.
 *
 * Exige sessão (o link sozinho não basta: o reset é da conta de quem está logado) e
 * valida a forma do token antes de renderizar. O token não aparece na tela nem em
 * mensagem de erro; expirado/uso repetido respondem a mesma recusa no POST.
 */
export default async function PinResetPage({
  searchParams,
}: {
  searchParams: Promise<Record<string, string | string[] | undefined>>;
}) {
  const session = await getSession();
  if (!session) redirect("/login");

  const params = await searchParams;
  const raw = params.token;
  const token = (Array.isArray(raw) ? raw[0] ?? "" : raw ?? "").trim();

  return (
    <>
      <SiteHeader current="/conta" />
      <main className="page page--narrow">
        <section className="card">
          <div className="card__head">
            <div>
              <h2 className="card__title">
                <KeyRound size={15} strokeWidth={1.8} aria-hidden="true" /> Redefinir segunda senha
              </h2>
              <p className="card__sub">
                O link vale por 15 minutos e só pode ser usado uma vez.
              </p>
            </div>
          </div>
          <div className="card__body">
            {validResetToken(token) ? (
              <>
                <p className="ui-field__hint" style={{ marginTop: 0 }}>
                  Ao confirmar, a segunda senha atual é apagada. No próximo acesso ao cliente do
                  jogo você cadastra o PIN novo no diálogo da segunda senha.
                </p>
                <ConfirmPinResetForm token={token} />
              </>
            ) : (
              <div className="empty">
                <strong>Link inválido ou incompleto</strong>
                <p>
                  Peça um link novo em <Link href="/conta">Minha conta</Link>. Se já usou este
                  link, ele não vale de novo.
                </p>
              </div>
            )}
          </div>
        </section>
      </main>
    </>
  );
}
