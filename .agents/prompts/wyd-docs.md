# Wyd Docs - v1.0.0

Este é um documento de referência para a versão 1.0.0 do Wyd Docs, que fornece informações detalhadas sobre a utilização e funcionalidades da ferramenta.
Todos os pontos levantados no documento devem ser finalizados até o fechamento da versão 1.0.0, garantindo que todas as funcionalidades estejam implementadas e testadas.

## Detalhes técnicos

A documentação é um projeto que utiliza o framework MKDocs, que é uma ferramenta de geração de documentação estática baseada em Markdown. O projeto é estruturado para facilitar a navegação e a compreensão das funcionalidades do Wyd Docs.

O Theme utilizado é o Material for MkDocs, que oferece uma interface moderna e responsiva, além de suporte a recursos avançados como pesquisa integrada, navegação lateral e personalização de estilos.

Toda a documentação deve ser escrita em PT-Br, com escrita clara e objetiva, utilizando exemplos práticos sempre que possível para ilustrar os conceitos abordados.

Os documentos devem usar recursos ricos, emojis, cores, links e imagens para tornar a leitura mais agradável e informativa. A utilização de exemplos de código, diagramas e capturas de tela é incentivada para facilitar a compreensão dos usuários.

Ter cuidado ao desenhar a estrutura de navegação, garantindo que os tópicos estejam organizados de forma lógica e intuitiva. A documentação deve ser revisada regularmente para manter a precisão e a relevância das informações.

Cuidado ao desenhar fluxos e diagramas, garantindo que eles sejam claros e representem fielmente os processos descritos. A documentação deve ser mantida atualizada com as últimas mudanças e melhorias do Wyd Docs.

### Extensões Mkdocs

- admonition
- codehilite
- pymdownx.details
- pymdownx.tasklist
- pymdownx.emoji

### Plugins Mkdocs

- search
- mkdocs-material
- social
- mkdocs-minify-plugin
- mkdocs-git-revision-date-plugin
- autorefs

## Estrutura de Navegação

A estrutura de navegação da documentação deve ser organizada de forma hierárquica, permitindo que os usuários encontrem facilmente as informações que procuram. A seguir, apresentamos uma sugestão de estrutura de navegação para o Wyd Docs:

```Text
- arquitetura
    - adr
        - [numero-adr]-[titulo-adr].md - Documentação de decisões arquiteturais (ADR) do projeto, incluindo o número e o título da decisão.
    - wyd-admin
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, arquitetura, fluxos, diagramas, etc.
    - wyd-bots
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, arquitetura, fluxos, diagramas, etc.
    - wyd-core
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, arquitetura, fluxos, diagramas, etc.
    - wyd-db
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, arquitetura, fluxos, diagramas, etc.
    - wyd-launcher
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, arquitetura, fluxos, diagramas, etc.
    - wyd-net
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, arquitetura, fluxos, diagramas, etc.
    - wyd-protocol
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, arquitetura, fluxos, diagramas, etc.
    - wyd-server
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, arquitetura, fluxos, diagramas, etc.
    - wyd-sim
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, arquitetura, fluxos, diagramas, etc.
    - wyd-portal
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, arquitetura, fluxos, diagramas, etc.
    - wyd-client
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, arquitetura, fluxos, diagramas, etc.
    - wyd-tools
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, arquitetura, fluxos, diagramas, etc.
    - [Pastas e arquivos da arquitetura completa do projeto como um todo].md - Documentação a nivel global de arquitetura, fluxos, diagramas, etc.
    - index.md
- manual-desenvolvimento
    - wyd-admin
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-bots

    - wyd-core

    - wyd-db

    - wyd-launcher

    - wyd-net
 
    - wyd-protocol

    - wyd-server

    - wyd-sim

    - wyd-portal

    - wyd-client

    - wyd-tools

- manual-usuario
    - wyd-admin
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-bots

    - wyd-core

    - wyd-db

    - wyd-launcher

    - wyd-net
 
    - wyd-protocol

    - wyd-server

    - wyd-sim

    - wyd-portal

    - wyd-client

    - wyd-tools
- manual-testes (incluir pastas por tipo de testes, como é testado cada parte do projeto, como é a integração de testes, como é feito o deploy de testes, etc.)
    - wyd-admin
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-bots

    - wyd-core

    - wyd-db

    - wyd-launcher

    - wyd-net
 
    - wyd-protocol

    - wyd-server

    - wyd-sim

    - wyd-portal

    - wyd-client

    - wyd-tools
- manual-administracao
    - wyd-admin
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-bots

    - wyd-core

    - wyd-db

    - wyd-launcher

    - wyd-net
 
    - wyd-protocol

    - wyd-server

    - wyd-sim

    - wyd-portal

    - wyd-client

    - wyd-tools
- agente-ia-freebuff
    - freebuff
        - instalação.md - Guia de instalação do Freebuff, incluindo requisitos do sistema, etapas de instalação e configuração.
        - configuração.md - Guia de configuração do Freebuff, detalhando as opções de personalização e ajustes disponíveis.
    - skills
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc. - está pasta deve documentar a pasta .agents/skills, que contem as skills de IA do projeto.
    - prompts
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc. - está pasta deve documentar a pasta .agents/prompts, que contem os prompts de IA do projeto.
    - fluxo-agente-ia.md - Documentação detalhada do fluxo atual de funcionamento do agente de IA, incluindo o uso das skills, fluxo de execução do agente, exemplos de interação com o usuário, etc.
    - [outras docs do agente de IA].md - Documentação adicional relacionada ao agente de IA, abordando tópicos específicos, melhorias futuras e considerações de design.
- index.md
```