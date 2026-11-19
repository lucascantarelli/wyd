/* Comportamento do site: tema, menu, filtro, avisos, cópia e destaque de código. */
(function () {
  "use strict";

  var raiz = document.documentElement;

  document.querySelectorAll("[data-tema-botao]").forEach(function (botao) {
    botao.addEventListener("click", function () {
      raiz.dataset.tema = raiz.dataset.tema === "claro" ? "escuro" : "claro";
      localStorage.setItem("wyd-tema", raiz.dataset.tema);
    });
  });

  var menu = document.querySelector("[data-menu]");
  var barra = document.querySelector("[data-barra]");

  if (menu && barra) {
    menu.addEventListener("click", function () {
      var aberta = barra.classList.toggle("aberta");
      menu.setAttribute("aria-expanded", String(aberta));
    });
  }

  var filtro = document.querySelector("[data-filtro]");
  var navegacao = barra ? barra.querySelector(".navegacao") : null;

  if (filtro && navegacao) {
    filtro.addEventListener("input", function () {
      var termo = filtro.value.trim().toLowerCase();
      var links = navegacao.querySelectorAll("a");

      if (!termo) {
        navegacao.querySelectorAll("li").forEach(function (li) {
          li.hidden = false;
        });
        links.forEach(function (link) {
          link.hidden = false;
        });
        return;
      }

      links.forEach(function (link) {
        link.hidden = link.textContent.toLowerCase().indexOf(termo) === -1;
      });
      navegacao.querySelectorAll("li").forEach(function (li) {
        li.hidden = !li.querySelector("a:not([hidden])");
      });
    });
  }

  document.querySelectorAll(".navegacao a").forEach(function (link) {
    var destino = link.getAttribute("href");
    if (!destino || destino.charAt(0) !== "/") {
      return;
    }
    var atual = window.location.pathname.replace(/index\.html$/, "");
    if (atual === destino || (destino !== "/" && atual.indexOf(destino) === 0 && destino.slice(-1) === "/")) {
      link.classList.add("ativo");
    }
  });

  document.querySelectorAll(".pagina blockquote").forEach(function (citacao) {
    var primeiro = citacao.querySelector("p");
    if (!primeiro) {
      return;
    }
    var casamento = primeiro.textContent.trim().match(/^\[!([A-Z]+)\]\s*(.*)$/);
    if (!casamento) {
      return;
    }

    var aviso = document.createElement("div");
    aviso.className = "aviso aviso-" + casamento[1].toLowerCase();

    var titulo = casamento[2].trim();
    if (titulo) {
      var elemento = document.createElement("p");
      elemento.className = "aviso-titulo";
      elemento.textContent = titulo;
      aviso.appendChild(elemento);
    }

    var corpo = document.createElement("div");
    corpo.className = "aviso-corpo";

    primeiro.remove();
    var irmao = citacao.firstElementChild;
    while (irmao) {
      var proximo = irmao.nextElementSibling;
      corpo.appendChild(irmao);
      irmao = proximo;
    }
    if (!corpo.firstElementChild) {
      corpo.textContent = citacao.textContent.trim();
    }

    aviso.appendChild(corpo);
    citacao.replaceWith(aviso);
  });

  document.querySelectorAll(".pagina pre").forEach(function (bloco) {
    if (bloco.classList.contains("mermaid")) {
      return;
    }
    var botao = document.createElement("button");
    botao.type = "button";
    botao.className = "botao-copiar";
    botao.textContent = "copiar";
    botao.addEventListener("click", function () {
      var codigo = bloco.querySelector("code");
      var texto = codigo ? codigo.innerText : bloco.innerText;
      navigator.clipboard.writeText(texto).then(function () {
        botao.textContent = "copiado";
        window.setTimeout(function () {
          botao.textContent = "copiar";
        }, 1500);
      });
    });
    bloco.appendChild(botao);
  });

  var blocos = Array.prototype.slice.call(document.querySelectorAll(".pagina pre code:not(.language-mermaid)"));
  if (blocos.length) {
    var script = document.createElement("script");
    script.src = "https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.12.0/build/highlight.min.js";
    script.onload = function () {
      blocos.forEach(function (bloco) {
        window.hljs.highlightElement(bloco);
      });
    };
    document.head.appendChild(script);
  }
})();
