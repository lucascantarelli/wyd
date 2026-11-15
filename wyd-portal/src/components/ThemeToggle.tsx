"use client";

import { Moon, Sun } from "lucide-react";
import { useEffect, useState } from "react";

const CHAVE = "wyd-tema";

/**
 * Alterna entre o tema escuro (obsidiana) e o claro (pergaminho).
 *
 * O estado real vive no atributo `data-tema` do `<html>`, escrito **antes da pintura** pelo
 * script inline do `layout.tsx`. Por isso o componente só lê o documento depois de montar:
 * guardar o tema em estado React antes da hidratação piscaria a tela no tema errado.
 *
 * A troca mexe apenas na camada `--skin-*` do CSS — os tokens de `identidade.tokens`
 * (contrato com o launcher) não mudam.
 */
export function ThemeToggle() {
  const [claro, setClaro] = useState(false);

  useEffect(() => {
    setClaro(document.documentElement.dataset.tema === "claro");
  }, []);

  function alternar() {
    const proximo = !claro;
    setClaro(proximo);
    document.documentElement.dataset.tema = proximo ? "claro" : "escuro";
    try {
      localStorage.setItem(CHAVE, proximo ? "claro" : "escuro");
    } catch {
      // Armazenamento indisponível (modo privado): vale só nesta sessão.
    }
  }

  const rotulo = claro ? "Usar tema escuro" : "Usar tema claro";

  return (
    <button
      type="button"
      className="icon-btn"
      onClick={alternar}
      aria-label={rotulo}
      aria-pressed={claro}
      title={rotulo}
    >
      {claro ? (
        <Moon size={17} strokeWidth={1.8} aria-hidden="true" />
      ) : (
        <Sun size={17} strokeWidth={1.8} aria-hidden="true" />
      )}
    </button>
  );
}
