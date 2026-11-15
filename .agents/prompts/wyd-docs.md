# Wyd Docs - v1.0.0

FASE ATUAL: PLANEJAMENTO
INICIO: 2024-06-10
ATIVIDADE: análise, planejamento e melhoria deste arquivo de documentação, para temos metas definidas e um plano de ação para a versão 1.0.0 do Wyd Docs.

Este é um documento de referência para a versão 1.0.0 do Wyd Docs, que fornece informações detalhadas sobre a utilização e funcionalidades da ferramenta.
Todos os pontos levantados no documento devem ser finalizados até o fechamento da versão 1.0.0, garantindo que todas as funcionalidades estejam implementadas e testadas.

todo o projeto novo deve ser executado dentro da pasta docs-new

- todos os arquivos de configuração devem está dentro da pasta docs-new, incluindo o arquivo mkdocs.yml, que é o arquivo de configuração principal do projeto.

## Detalhes técnicos

A documentação é um projeto que utiliza o framework MKDocs, que é uma ferramenta de geração de documentação estática baseada em Markdown. O projeto é estruturado para facilitar a navegação e a compreensão das funcionalidades do Wyd Docs.

O Theme utilizado é o Material for MkDocs, que oferece uma interface moderna e responsiva, além de suporte a recursos avançados como pesquisa integrada, navegação lateral e personalização de estilos.

Toda a documentação deve ser escrita em PT-Br, com escrita clara e objetiva, utilizando exemplos práticos sempre que possível para ilustrar os conceitos abordados.

Os documentos devem usar recursos ricos, emojis, cores, links e imagens para tornar a leitura mais agradável e informativa. A utilização de exemplos de código, diagramas e capturas de tela é incentivada para facilitar a compreensão dos usuários.

Ter cuidado ao desenhar a estrutura de navegação, garantindo que os tópicos estejam organizados de forma lógica e intuitiva. A documentação deve ser revisada regularmente para manter a precisão e a relevância das informações.

Cuidado ao desenhar fluxos e diagramas, garantindo que eles sejam claros e representem fielmente os processos descritos. A documentação deve ser mantida atualizada com as últimas mudanças e melhorias do Wyd Docs.

## Diretrizes do projeto
- Extensões Mkdocs
    - admonition
    - codehilite
    - pymdownx.details
    - pymdownx.tasklist
    - pymdownx.emoji

- Plugins Mkdocs
    - search
    - mkdocs-material
    - social
    - mkdocs-minify-plugin
    - mkdocs-git-revision-date-plugin
    - autorefs

## Estrutura de Navegação

A estrutura de navegação da documentação deve ser organizada de forma hierárquica, permitindo que os usuários encontrem facilmente as informações que procuram. A seguir, apresentamos uma sugestão de estrutura de navegação para o Wyd Docs:

```Text
- arquitetura - O foco dessa documentação é para arquitetos, incluindo diagramas de arquitetura, decisões arquiteturais, padrões de design, fluxos de dados, etc.
    - adr - atualmente deve está muito desorganizada, devemos reorganizar, analisar redundancias, e criar uma estrutura de navegação mais clara e intuitiva, remover ruidos e melhorar a clareza da documentação.
        - [tipo-adr]-[numero-adr]-[titulo-adr].md - Documentação de decisões arquiteturais (ADR) do projeto, incluindo o número e o título da decisão.
    - [Pastas e arquivos da arquitetura completa do projeto como um todo].md - Documentação a nivel global de arquitetura, fluxos, diagramas, etc.
- manual-desenvolvimento - o foco dessa documentação é para desenvolvedores, incluindo guias de desenvolvimento, padrões de código, fluxos de trabalho, exemplos de implementação, etc.
    - [Pastas e arquivos da arquitetura completa do projeto como um todo].md - Documentação a nivel global de boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-admin
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-bots
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-core
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-db
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-launcher
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-net
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-protocol
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-server
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-sim
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-portal
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-client
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-tools
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
    - wyd-docs
        - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
- manual-usuario
    - [Pastas e arquivos de manual do usuário].md - Documentação a nivel de para moderadores e administradores do sistema, incluindo guias de uso, tutoriais, dicas e melhores práticas, guias de fluxos administrativos do portal, descrição dos papeis de cada tipo de usuário, mapeamento de descrição de todas as funcionalidades do portal, etc, o foco dessa documentação é para os moderadores e administradores do sistema, não para os usuários finais.
- manual-testes (incluir pastas por tipo de testes, como é testado cada parte do projeto, como é a integração de testes, como é feito o deploy de testes, etc.) - o foco dessa documentação é para testadores, incluindo guias de testes, estratégias de teste, casos de teste, resultados de teste, ferramentas de teste, etc.
    - [Pastas e arquivos do projeto].md - Documentação a nivel de projeto, boas praticas, padrões de código, fluxos, exemplos, etc.
- agente-ia-freebuff - o foco dessa documentação é para desenvolvedores, incluindo guias de desenvolvimento, padrões de código, fluxos de trabalho, exemplos de implementação, etc.
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

## Tarefas A serem executadas

- Analisar todo o projeto atual mkdocs e identificar pontos de melhoria na estrutura de navegação, organização dos tópicos, clareza da documentação, redundâncias, ruidos e inconsistências, plugins, extensões, etc.
- Reescrever a seção de ADR, reorganizando os tópicos, removendo redundâncias e melhorando a clareza da documentação.
- Reescrever a seção deste documento ## Estrutura de Navegação com toda a estrutura de navegação planejada, incluindo pastas, arquivos, indices. substituir os apontamentos descritos por pastas e arquivos reais a serem criados no projeto, garantindo que a estrutura seja clara, intuitiva e fácil de navegar.
- Criar uma seção neste documento de ## Análise da poposta e sugestão de melhorias, detalhando as mudanças propostas, justificando as escolhas de organização e estrutura, e apresentando sugestões de melhorias adicionais para a documentação, criticando escolhas, propondo alternativas, etc.
- Aguardar revisão e alterações sugeridas e atualizar esta documentação conforme necessário, garantindo que todas as mudanças sejam implementadas e testadas antes do fechamento da versão 1.0.0.
- aguardar aprovação do documento final e das alterações sugeridas, garantindo que todas as mudanças sejam implementadas e testadas antes do fechamento da versão 1.0.0.

## Análise da proposta e sugestão de melhorias

