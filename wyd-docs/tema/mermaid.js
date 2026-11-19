/* Renderiza as cercas ```mermaid``` que o Markdown do MkDocs 2.0 entrega como código. */
import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@12.3.0/dist/mermaid.esm.min.mjs";

const escuro = document.documentElement.dataset.tema !== "claro";

mermaid.initialize({
  startOnLoad: false,
  securityLevel: "strict",
  theme: escuro ? "dark" : "default",
});

document.querySelectorAll("pre code.language-mermaid").forEach((codigo) => {
  const bloco = document.createElement("div");
  bloco.className = "mermaid";
  bloco.textContent = codigo.textContent;
  const pre = codigo.closest("pre");
  if (pre) {
    pre.replaceWith(bloco);
  }
});

await mermaid.run({ querySelector: ".mermaid" });
