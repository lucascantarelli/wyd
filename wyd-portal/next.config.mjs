/** @type {import('next').NextConfig} */
const nextConfig = {
  // A imagem do `wyd-ops` copia `.next/standalone`; sem isto o runtime precisaria do
  // `node_modules` inteiro. O contrato do portal (BFF, login, rotas) é do planejamento
  // em `planejamento-1.0.0/wyd-portal/` e não está implementado nesta estrutura inicial.
  output: "standalone",
};

export default nextConfig;
