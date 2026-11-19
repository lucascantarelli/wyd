# Informnações sobre o projeto WYD

Estamos planejando a versão v1.0.0 do projeto WYD, e para isso estamos criando uma documentação de planejamento, que será utilizada para o desenvolvimento do projeto WYD.
WYD2 é um antigo jogo disbribuido no brasil pela Ongame, e o projeto WYD é um projeto de reimplementação do jogo WYD2, com o objetivo de criar uma versão moderna e atualizada do jogo, com novas funcionalidades, melhorias de desempenho, e etc.

temos uma pasta `deprecated`, conte um projeto em rust, usado para teste, está com muito bug, tudo está quebrado, nenhum sistema funciona, e não é mais utilizado, então ele está deprecated, e não será mais desenvolvido, ele está apenas para referência, para que possamos entender como o jogo funcionava, e como podemos melhorar o projeto WYD, nele fizemos estudos dos pacotes do cliente, criamos uma documentação robusta de tudo, mas, nada funcionada, estava com sistemas mal planejados, modelagem de banco horrivel. vamos vboltar para planejar novamente tudo do zero, com planejamento inicial, documentação, validação e depois sim, vamos desenvolver, e não o contrário, como foi feito no projeto deprecated.

## Diretório de tabalho
Atualmente devemos trabalhar no diretório `planejamento-1.0.0`, dentro dele consta o diretório de documentação de planejamento de todos os sistemas que iremos desenvolver.

O projeto piloto do planejamento é o `wyd-db`, que é o sistema de banco de dados do projeto WYD, e é um dos sistemas mais importantes do projeto, pois ele é responsável por armazenar todas as informações do jogo, como personagens, itens, monstros, etc.
Nele estamos definindo como será a estrutura do banco de dados, e como será a comunicação entre os outros sistemas do projeto WYD, estruturas de pastas, modelos de documentação, e etc.

Cada diretório é um sistema do projeto WYD, e cada sistema tem sua documentação de planejamento, nosso primeiro passo é entender cada sistema, e depois podemos começar a desenvolver.
- `wyd-admin`: Documentação de planeamento do projeto de administração do projeto WYD.
- `wyd-bots`: Documentação de planeamento do projeto de bots do projeto WYD.
- `wyd-client`: Documentação de planeamento do projeto de cliente do projeto WYD. projeto responsável por possivel modificações feitas no cliente do jogo, como por exemplo, adicionar novos pacotes, modificar pacotes existentes, adicionar novos arquivos, modificar arquivos existentes, etc, ele deve ser o projeto que irá gerar os patchs do cliente do jogo, e também deve ser o projeto que irá gerar os arquivos de configuração do cliente do jogo, como por exemplo, serverlist, itemlist, que serão baixados pelo launcher do jogo.
- `wyd-core`:  Documentação de planeamento do projeto de núcleo do projeto WYD.
- `wyd-db`: Documentação de planeamento do projeto de banco de dados do projeto WYD
- `wyd-docs`: Documentação de planeamento do projeto de documentação do projeto WYD. projeto responsável por toda a documentação do projeto WYD, como manuais, arquitetura, padrões, etc.
- `wyd-launcher`: Documentação de planeamento do projeto de launcher do projeto WYD. projeto responsável pelo launcher do jogo, responsável por baixar o jogo, configurarções do jogo, garantir os arquivos de serverlist,itemlist,entre outros arquivos atualizados com o servidor, baixar patches, etc.
- `wyd-net`: Documentação de planeamento do projeto de rede do projeto WYD.
- `wyd-portal`: Documentação de planeamento do projeto de portal do projeto WYD. projeto responsável por toda a parte de portal do projeto WYD, como site, login, cadastro, etc.
    - `wyd-portal/wyd-portal-prompt.md`: Prompt com orientações de como deve ser o portal do projeto WYD, com orientações de como deve ser o portal, como deve ser a interface, como deve ser a experiência do usuário, etc.
- `wyd-protocol`: Documentação de planeamento do projeto de protocolo do projeto WYD. projeto responsável por toda a parte de protocolo do projeto WYD, como pacotes, mensagens, etc.
- `wyd-server`: Documentação de planeamento do projeto de servidor do projeto WYD.
- `wyd-sim`: Documentação de planeamento do projeto de simulação do projeto WYD. projeto responsável por toda a matemática do jogo, como cálculos de dano, defesa, etc.
- `wyd-ops`: Documentação de planeamento do projeto de operações do projeto WYD. projeto responsável por toda a parte de operações do projeto WYD, ci, dockerfiles, deploy, monitoramento, etc.

### Diretório de do wyd-db
Como projeto piloto, já iniciamos a construção de algumas documentações e referencias do planejamento do `wyd-db`, e estamos organizando da melhor forma possível, para que possamos ter uma visão geral do projeto WYD.
vou esplicar o que é cada documento que estamos construindo, e como ele se relaciona com os outros documentos.
ATENÇÂO: Tivemos um problema grave com um modelo anterior, ele incluir muita referencia do projeto legado como verdade, por exemplo, só tinhamos fechado 2 adrs(0001 e 0002), ele criou diversas a mais, baseado no projeto antigo, analise isso.
diretório principal: `planejamento-1.0.0/wyd-db/`

- `docs/`: Diretório onde excreveremos o planejamento da documentação do `wyd-db`, essa é a documentação final do projeto MKDocs `wyd-docs`, ela não é documentação de referência, ela é a documentação final do projeto WYD, e será utilizada para o desenvolvimento do projeto WYD, ela será a documentação de manuais,arquitetura,padrões, cada sistema deve ter a sua documentação.
- `migrations/`: Diretório onde excreveremos o planejamento das migrations do `wyd-db`, dentro dessa pasta, devemos ter a documentação de cada migration que será criado no projeto `wyd-db`, cada migration deve ter sua documentação, futuramente, essas migrations serão desenvolvidas dentro do projeto `wyd-db`, e serão utilizadas para criar o banco de dados do projeto WYD, e também para atualizar o banco de dados do projeto WYD, quando houver alguma alteração na estrutura do banco de dados.
- `prompts`: Diretórios de prompts para ser usado no desenvolvimento do projeto, prompts exclusivos para o projeto `wyd-db`, voltados somente para esse projeto, não criar prompts genéricos.
- `skills`: Diretório de skills para ser usado no desenvolvimento do projeto, skills exclusivos para o projeto `wyd-db`, voltados somente para esse projeto, não criar skills genéricos.
- `src`: Diretório onde excreveremos o planejamento do código do `wyd-db`, dentro dessa pasta, devemos ter a documentação de cada arquivo que será criado, replicando a organização em pastas do projeto `wyd-db`, cada arquivo deve ter sua documentação, futuramente, esses arquivos serão desenvolvidos dentro do projeto `wyd-db`, estes documentos devem servir de referencia para o desenvolvimento do projeto `wyd-db`, deve conter todos os detalhes do arquivo, como funções, classes, métodos, variáveis, etc, o desenvolvedor deve ser capaz de executar completamente o desenvolvimento do arquivo, apenas com a documentação, sem precisar de mais nada, apenas a documentação.
- `tests`: Diretório onde excreveremos o planejamento dos testes do `wyd-db`, dentro dessa pasta, devemos ter a documentação de cada teste que será criado no projeto `wyd-db`, cada teste deve ter sua documentação, futuramente, esses testes serão desenvolvidos dentro do projeto `wyd-db`, e serão utilizados para testar o código do projeto `wyd-db`, e também para garantir que o código do projeto `wyd-db` está funcionando corretamente.
- `wyd-db-1.0.0.md`: Documento de planejamento do projeto `wyd-db`, nele temos a visão geral do projeto,responsabilidades do projeto, ele deve ser um dashboard do projeto, com metricas, tabelas, porcentagens do desenvolvimento do planejamento do projeto, deve conter os checklists de todas as issues que serão abertas para a execução do projeto, ele não é um documento onde devemos anotar decisões, regras, ele é o dashboard de acompanhamento do planejmaneto de `wyd-db`, ele deve conter todas as informações do projeto, como status, métricas, porcentagens, checklists, etc.

## Documentação de referência
Atualmente temos muitos pontos de referência, isso pede está confundido o agente, mas vamos tentar organizar da melhor forma possível.

Temos o projeto teste `deprecated`, que é um projeto antigo, que foi desenvolvido como um protótipo, mas não é mais utilizado, ele está deprecated, e não será mais desenvolvido, ele está apenas para referência, para que possamos entender como o jogo funcionava, e como podemos melhorar o projeto WYD.

Temos um projeto em go, que tem referencias do antigo servidor em C++, refatoração para GO, para referencias de funcionamento: https://github.com/Jean1dev/w2pp-OpenWYD

- `.agents\prompts\análises`: Diretório de pre-análise, validações, provas, checkagem, discursões, definições de regras, divergencias, aqui onde planejamos, encontramos inconsistencias, pontos de melhorias, e etc, e daqui que teremos referencias de onde erramos, o que precisa melhorar, e etc, é um diretório de referencia, não é um diretório de planejamento, ele é um diretório de referencia para o planejamento.
- `crates\wyd-admin`: Primeiro projeto de conceito desenvolvido, o conteudo atual será totalmente reescrito, e o projeto será reestruturado, mas futuramente tudo que for planejado em `planejamento-1.0.0/wyd-admin/` será desenvolvido dentro do projeto `crates\wyd-admin`.
- `crates\wyd-bots`: Primeiro projeto de conceito desenvolvido, o conteudo atual será totalmente reescrito, e o projeto será reestruturado, mas futuramente tudo que for planejado em `planejamento-1.0.0/wyd-bots/` será desenvolvido dentro do projeto `crates\wyd-bots`.
- `crates\wyd-core`: Primeiro projeto de conceito desenvolvido, o conteudo atual será totalmente reescrito, e o projeto será reestruturado, mas futuramente tudo que for planejado em `planejamento-1.0.0/wyd-core/` será desenvolvido dentro do projeto `crates\wyd-core`.
- `crates\wyd-db`: Primeiro projeto de conceito desenvolvido, o conteudo atual será totalmente reescrito, e o projeto será reestruturado, mas futuramente tudo que for planejado em `planejamento-1.0.0/wyd-db/` será desenvolvido dentro do projeto `crates\wyd-db`.
- `crates\wyd-launcher`: Primeiro projeto de conceito desenvolvido, o conteudo atual será totalmente reescrito, e o projeto será reestruturado, mas futuramente tudo que for planejado em `planejamento-1.0.0/wyd-launcher/` será desenvolvido dentro do projeto `crates\wyd-launcher`.
- `crates\wyd-net`: Primeiro projeto de conceito desenvolvido, o conteudo atual será totalmente reescrito, e o projeto será reestruturado, mas futuramente tudo que for planejado em `planejamento-1.0.0/wyd-net/` será desenvolvido dentro do projeto `crates\wyd-net`.
- `crates\wyd-protocol`: Primeiro projeto de conceito desenvolvido, o conteudo atual será totalmente reescrito, e o projeto será reestruturado, mas futuramente tudo que for planejado em `planejamento-1.0.0/wyd-protocol/` será desenvolvido dentro do projeto `crates\wyd-protocol`.
- `crates\wyd-server`: Primeiro projeto de conceito desenvolvido, o conteudo atual será totalmente reescrito, e o projeto será reestruturado, mas futuramente tudo que for planejado em `planejamento-1.0.0/wyd-server/` será desenvolvido dentro do projeto `crates\wyd-server`.
- `crates\wyd-sim`: Primeiro projeto de conceito desenvolvido, o conteudo atual será totalmente reescrito, e o projeto será reestruturado, mas futuramente tudo que for planejado em `planejamento-1.0.0/wyd-sim/` será desenvolvido dentro do projeto `crates\wyd-sim`.
- `wyd-portal`: Primeiro projeto de conceito desenvolvido, o conteudo atual será totalmente reescrito, e o projeto será reestruturado, mas futuramente tudo que for planejado em `planejamento-1.0.0/wyd-portal/` será desenvolvido dentro do projeto `wyd-portal`.
- `docs`: Primeiro projeto de conceito desenvolvido, o conteudo atual será totalmente reescrito, e o projeto será reestruturado, mas futuramente tudo que for planejado em `planejamento-1.0.0/wyd-docs/` será desenvolvido dentro do projeto `wyd-docs`.
    - `docs\wyd`: Aqui temos referencias do funcionamento completo do jogo, historias, sistemas, regras, etc, é um diretório de referencia, não é um diretório de planejamento, ele é um diretório de referencia para o planejamento.
- `deprecated`: Diretório raiz do projeto depreciado, ele contem todos os arquivos de configuração do projeto, como `Cargo.toml`, `rust-toolchain.toml`, `Makefile`, `Dockerfile`, etc, ele é um diretório de referencia, não é um diretório de planejamento, ele é um diretório de referencia para o planejamento.

## Conclusão
O projeto WYD é um projeto complexo, com diversos sistemas interligados, e cada sistema tem sua própria documentação de planejamento. O foco atual é finalizar o planejamento do `wyd-db`, que servirá como base para o desenvolvimento dos outros sistemas. A documentação de referência e os diretórios de trabalho estão organizados para facilitar a compreensão e o desenvolvimento do projeto como um todo.

duvidas sobre  me pergunte, vou tentar ser o mais claro possível para que possamos avançar no projeto WYD de forma eficiente e organizada.

## Obrigatoriedades

- Primeira atividade é analisar completamente o cenário do projeto WYD, entender o que está planejado, o que está em desenvolvimento, e o que está em produção.
- Atualizar a documentação de IA: `AGENTS.md`, `.agents/skills`, `.agents/prompts`, `.agents/memory` e etc, para que possamos ter uma visão geral do projeto WYD.
- Vamos primeiramente analisar organização de pastas, estrutura respeitando o planejamento, e depois vamos analisar cada sistema, e depois vamos analisar cada arquivo, e depois vamos analisar cada função, e depois vamos analisar cada linha de código.
- Vamos analisar cada sistema, e depois vamos analisar cada arquivo, e depois vamos analisar cada função, e depois vamos analisar cada linha de código.
- Existe algumas definições já estabelecidas no planejamento, devemos seguir os padrões definidos, e não criar novos padrões, a não ser que seja extremamente necessário, e que seja aprovado pelo time de desenvolvimento.
- primeiro iremos planejar completamente, devemos ter uma visão geral do projeto WYD, e depois sim, iremos desenvolver, não o contrário, como foi feito no projeto deprecated.
- Primeiro planejar organização das pastas de código e testes, respeitando boas praticas de desenvolvimento e testes,SOLID,design patterns, clean code, clean architecture, etc, e depois sim, planejar cada sistema, e depois sim, planejar cada arquivo, e depois sim, planejar cada função, e depois sim, planejar cada linha de código.
- Planejar os testes seguindo boas praticas, piramide, TDD,BDD, coverge minimo de 85%, teste unitários/integração e e2e(portal e client), e2e(portal e client) devem ser feitos com playwright, e2e(server) deve ser feito com rust, e2e(db) deve ser feito com rust, e2e(launcher) deve ser feito com rust, e2e(bots) deve ser feito com rust, e2e(core) deve ser feito com rust, e2e(protocol) deve ser feito com rust, e2e(sim) deve ser feito com rust, e2e(net) deve ser feito com rust, e2e(admin) deve ser feito com rust.
- organizar o projeto WYD de forma que possamos ter uma visão geral do projeto, e que possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo.
- Planeje toda a comunicação entre os sistemas, como eles irão se comunicar, quais protocolos irão utilizar, quais mensagens irão enviar, quais mensagens irão receber, etc.
- Temos todo o sistema de monitoramento do projeto WYD, ele é um sistema complexo, que irá monitorar todos os sistemas do projeto WYD, e irá gerar alertas para o time de desenvolvimento, caso algum sistema esteja com problemas, ou esteja fora do ar, ele já está devidamente configurado na pasta `deprecated`
- IMPORTANTE: sempre que consultar a documentação de referencia, não copie nada, entenda e replaneje para a nova versão do projeto WYD, não copie nada do projeto deprecated, ele é apenas para referência, e não deve ser utilizado como base para o desenvolvimento do projeto WYD.

## Metas da versão 1.0.0

- Todos os projetos funcionando 100%, com testes unitários, integração e e2e, com cobertura mínima de 85%.
- Todos os projetos com documentação completa, com manuais, arquitetura, padrões, etc.
- Todas as funcionalidades do jogo implementadas, com testes unitários, integração e e2e, com cobertura mínima de 85%.
- Todos os projetos com monitoramento completo, com alertas para o time de desenvolvimento, caso algum sistema esteja com problemas, ou esteja fora do ar.
- Todos os projetos com deploy automatizado, com CI/CD, para que possamos ter uma visão geral do projeto WYD, e possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo.
- Todas as boas praticas  de desenvolvimento, testes, arquitetura, padrões, etc, implementadas em todos os projetos do projeto WYD.
- Todos os projetos com documentação de planejamento completa, com manuais, arquitetura, padrões, etc, para que possamos ter uma visão geral do projeto WYD, e possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo.
- Sistema de bots completamente funcional, aparecendo dentro do cliente, integragindo com players, executando ações,quests, etc, com testes unitários, integração e e2e, com cobertura mínima de 85%.
- Sistema de administração completamente funcional, com painel web, com testes unitários, integração e e2e, com cobertura mínima de 85%.
- Ci completamente funcional, com deploy automatizado, para que possamos ter uma visão geral do projeto WYD, e possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo.
- Ci criado completamente usando github actions customizadas, com jobs reutilizáveis, para que possamos ter uma visão geral do projeto WYD, e possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo.
- o Ci deve ser compostos por actions complexas, compostas de outras actions, para criação de workflows complexos, devemos ter um CI robusto, organizado, coerente e profissional
- o projeto MKdocs deve fazer uso completo de todos os recursos adicionas,plugins,extensões,emojis,cores, recursos ricos, etc, para que possamos ter uma visão geral do projeto WYD, e possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo, deve ser desenvolvimento profissional, organizado, coerente e robusto, com documentação completa, com manuais, arquitetura, padrões, etc.
- o projeto do portal deve seguir os padrões definidos no documento
- `wyd-client/files/client/`: Cliente do jogo, ele é usado para fazermos pesquisas, extratir informações, e etc.
- `wyd-client/files/client.zip`: Cliente do jogo compactado, ele é usado para teste do launcher.


## Propostas, melhorias, sugestões, ideias, etc
- Voce deve propor melhorias para o projeto WYD, mas não deve implementar nada sem antes discutir com o time de desenvolvimento, e sem antes ter a aprovação do time de desenvolvimento.
- Melhorias de boas praticas de código, SOLID, design patterns, clean code, clean architecture, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de arquitetura, padrões, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de funcionalidades, sistemas, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de documentação, manuais, arquitetura, padrões, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de testes, cobertura, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de monitoramento, alertas, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de deploy, CI/CD, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de organização de pastas, estrutura de código, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de comunicação entre sistemas, protocolos, mensagens, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de performance, otimizações, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de segurança, criptografia, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de usabilidade, interface, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de experiência do usuário, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de acessibilidade, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias de compatibilidade, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias no Sistema de bots, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.
- Melhorias no Sistema de administração, etc, devem ser discutidas com o time de desenvolvimento, e devem ser aprovadas pelo time de desenvolvimento antes de serem implementadas.

## Github - Reposítório

Nosso repositório do projeto WYD está hospedado no GitHub, e é o local onde iremos versionar todo o código do projeto WYD, e também onde iremos versionar toda a documentação do projeto WYD.

Atualmente não temos nada configurado, precisamos ter todo o projeto devidamente registrado e organizado no github, como um scrum master, vamos seguir metodologias ageis de desenvolvimento, como scrum, kanban, etc, para que possamos ter uma visão geral do projeto WYD, e possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo.

Nosso projeto do wyd não está com nada criado, projeto, milestones,labels, etc, nada do projeto está criado, precisamos criar tudo do zero, e organizar da melhor forma possível, para que possamos ter uma visão geral do projeto WYD, e possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo.

- Pesquisar boas praticas de gestão de projetos no github, devemos deixar todo nosso repositório publico, não devemos usar recursos pagos, tudo deve ser gratuito.
- Pesquisas boas praticas de issues, milestones, labels, projects, etc, devemos deixar todo nosso repositório publico, não devemos usar recursos pagos, tudo deve ser gratuito.
- Pesquisar boas praticas de organização de repositórios no github, devemos deixar todo nosso repositório publico, não devemos usar recursos pagos, tudo deve ser gratuito.
- Analisar a pratica de uso de runner self-hosted do github actions, vantagens, desvantagens, segurança, performance, etc, devemos deixar todo nosso repositório publico, não devemos usar recursos pagos, tudo deve ser gratuito.

## Deploys e Servidores
Atualmente não temos nenhum servidor configurado, precisamos configurar todo o ambiente de produção, homologação, desenvolvimento, testes, etc, para que possamos ter uma visão geral do projeto WYD, e possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo.
- Pesquisar boas praticas de deploy, devemos deixar todo nosso repositório publico, não devemos usar recursos pagos, tudo deve ser gratuito.
- Pesquisar onde podemos hospedar nossos serviços, usando serviços gratuitos que não precisem de cartão, como vercel, render, railway, supabase, fly.io, etc, devemos deixar todo nosso repositório publico, não devemos usar recursos pagos, tudo deve ser gratuito.

## Tecnologias, frameworks, linguagens, bibliotecas, etc

- Rust: Linguagem de programação principal do projeto WYD, utilizada para desenvolvimento de todos os sistemas do projeto WYD.
- Next.js: Framework utilizado para desenvolvimento do portal do projeto WYD.
- Tauri: Framework utilizado para desenvolvimento do launcher do projeto WYD.
- MKDocs: Framework utilizado para desenvolvimento da documentação do projeto WYD.
- Playwright: Framework utilizado para desenvolvimento dos testes e2e do projeto WYD.
- PostgreSQL: Banco de dados utilizado para desenvolvimento do projeto WYD.
- Docker: Ferramenta utilizada para desenvolvimento do projeto WYD, para criação de containers, e para deploy dos sistemas do projeto WYD.
- GitHub Actions: Ferramenta utilizada para desenvolvimento do projeto WYD, para criação de workflows, e para deploy dos sistemas do projeto WYD.
- GitHub Projects: Ferramenta utilizada para desenvolvimento do projeto WYD, para criação de boards, e para gerenciamento dos sistemas do projeto WYD.
- Todas as tecnologias devem está atualizadas para as versões mais recentes, e devem ser compatíveis entre si, para que possamos ter uma visão geral do projeto WYD, e possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo.
- as versões devem ser compatíveis entre si, para que possamos ter uma visão geral do projeto WYD, e possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo.
- Travar as versões na versão mais recente, para que possamos ter uma visão geral do projeto WYD, e possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo.'
- Usar boas praticas de segurança, devops, dockerfiles, docker-compose, etc, para que possamos ter uma visão geral do projeto WYD, e possamos entender como cada sistema se relaciona com os outros sistemas, e como cada sistema se relaciona com o projeto como um todo.
