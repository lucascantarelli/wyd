import "./globals.css";

export const metadata = {
  title: "WYD 1.0.0",
  description: "Portal do WYD 1.0.0 — estrutura inicial.",
};

export default function RootLayout({ children }) {
  return (
    <html lang="pt-BR">
      <body>{children}</body>
    </html>
  );
}
