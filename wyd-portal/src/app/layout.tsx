import type { Metadata } from "next";
import { Cinzel, Manrope } from "next/font/google";

import "./globals.css";
import "./publico.css";

/*
 * Duas famílias, dois papéis:
 * - Cinzel (serifada lapidar) assina títulos, marca e números de destaque;
 * - Manrope (sans geométrica) carrega a operação: tabelas, formulários, legendas.
 * Ambas são expostas como variáveis CSS e consumidas só por `globals.css`.
 */
const display = Cinzel({
  subsets: ["latin"],
  weight: ["500", "600", "700"],
  display: "swap",
  variable: "--font-display",
});

const sans = Manrope({
  subsets: ["latin"],
  weight: ["400", "500", "600", "700"],
  display: "swap",
  variable: "--font-sans",
});

export const metadata: Metadata = {
  title: "WYD2 · Administração",
  description: "Portal de administração do servidor WYD2",
};

/**
 * Aplica o tema **antes da primeira pintura**. Sem isso, quem escolheu o claro veria um flash
 * escuro a cada carregamento. A escolha salva vence; sem escolha, segue o sistema.
 */
const TEMA_INICIAL = `(function(){try{var t=localStorage.getItem("wyd-tema");if(t!=="claro"&&t!=="escuro"){t=window.matchMedia("(prefers-color-scheme: light)").matches?"claro":"escuro";}document.documentElement.dataset.tema=t;}catch(e){document.documentElement.dataset.tema="escuro";}})();`;

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html
      lang="pt-BR"
      className={`${display.variable} ${sans.variable}`}
      // `data-tema` é escrito pelo script abaixo antes da hidratação.
      suppressHydrationWarning
    >
      <head>
        <script dangerouslySetInnerHTML={{ __html: TEMA_INICIAL }} />
      </head>
      <body>{children}</body>
    </html>
  );
}
