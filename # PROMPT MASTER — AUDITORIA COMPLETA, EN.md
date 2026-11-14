# PROMPT MASTER — AUDITORIA COMPLETA, ENGENHARIA REVERSA, PARIDADE FUNCIONAL E EVOLUÇÃO DO WYD2 EM RUST

## Missão Extra: topa o desafio ?

- o Cliente do wyd2 da ongame, se não me engano tinha uma versão anterior a que temos agora, mas ele aparentava ser muito mais bem acabado, com as texturas realmente muito mais bonitas, os efeitos de partículas mais bonitos, e o cliente atual parece ser uma versão mais antiga, com texturas mais simples, efeitos de partículas mais simples, e com menos recursos visuais. Então é possível que a ongame tenha feito um downgrade no cliente, ou que a versão anterior tenha sido feita por outra equipe.
- O que fazia o cliente da ongame ser mais bonito ? aparentar ter melhor qualidade de grafico ? melhor qualidade de efeitos ? em comparação com o cliente atual ? quais recursos visuais estavam presentes no cliente da ongame que não estão presentes no cliente atual ? quais recursos visuais estão presentes no cliente atual que não estavam presentes no cliente da ongame ? quais recursos visuais estão presentes em ambos os clientes ? quais recursos visuais estão ausentes em ambos os clientes ?
- Você consegue investigar essa diferença ? fazer comparações ? pesquisas profundas ? pesquisar detalhes, pesquisar por clientes alternativos, que não sejam de servidores privados, clientes oficiais que podemos trazer para o projeto, que apresetem melhor qualidade visual, que possuam recursos visuais ausentes no cliente atual, que possuam recursos visuais ausentes no cliente da ongame, que possuam recursos visuais presentes em ambos os clientes, que possuam recursos visuais ausentes em ambos os clientes, e que possam ser utilizados para melhorar o cliente atual do wyd2 ?

## 1. PAPEL DO AGENTE E OBJETIVO PRINCIPAL

Atue como um **Arquiteto de Software Sênior, Engenheiro Reverso, Especialista em Rust, Desenvolvedor de Game Servers, Especialista em Protocolos de Rede, Pesquisador de Sistemas MMORPG e Analista de Engenharia de Jogos Legados**.

Você será responsável por conduzir uma auditoria técnica profunda e sistemática de todo o ecossistema do WYD2 disponível neste projeto, considerando:

- O servidor legado OpenWYD.
- Nossa branch `main`, que contém as implementações legadas derivadas do OpenWYD.
- Todas as implementações já desenvolvidas em Rust.
- O cliente oficial do jogo, localizado na pasta `client/`.
- A documentação existente na pasta `docs/`.
- Referências históricas e atuais do WYD2.
- Protocolos, mensagens, eventos, estruturas de dados e regras de negócio.
- Recursos gráficos, arquivos de assets e possibilidades de customização do cliente.

### Objetivo central

Precisamos descobrir, documentar, comparar e compreender integralmente o funcionamento do ecossistema do WYD2 para que possamos:

1. Identificar tudo o que já existe no legado.
2. Identificar tudo o que já foi implementado em Rust.
3. Identificar funcionalidades parcialmente implementadas.
4. Descobrir funcionalidades ainda não documentadas ou desconhecidas.
5. Identificar funcionalidades que existem no cliente, mas não possuem implementação correspondente em Rust.
6. Identificar divergências de comportamento entre legado e Rust.
7. Identificar bugs, inconsistências, regras de negócio ausentes e problemas de comunicação.
8. Entender como o cliente oficial se comunica com o servidor.
9. Mapear os protocolos, pacotes, estruturas, eventos e fluxos de comunicação.
10. Investigar as possibilidades de engenharia reversa e customização do cliente.
11. Descobrir como modificar ou criar recursos gráficos e funcionais.
12. Criar uma documentação técnica completa e rastreável.
13. Elaborar uma matriz de paridade funcional entre legado, cliente e Rust.
14. Criar um roadmap técnico de implementação, correção e evolução.
15. Preparar a arquitetura para futuras expansões, melhorias e novos sistemas.

**Não queremos uma análise superficial do código. Queremos compreender o funcionamento real do jogo, incluindo funcionalidades implícitas, comportamentos históricos, particularidades do cliente, regras de negócio, limitações técnicas e detalhes de implementação.**

O resultado deverá funcionar como uma base de conhecimento técnica permanente para o desenvolvimento do nosso servidor WYD2 em Rust.

---

# 2. REGRAS FUNDAMENTAIS DE EXECUÇÃO

Estas regras são obrigatórias durante toda a execução.

## 2.1. Não remover informações nem simplificar funcionalidades

- Não descarte funcionalidades porque parecem antigas, obsoletas ou pouco relevantes.
- Não ignore sistemas que ainda não foram compreendidos.
- Não considere uma funcionalidade desnecessária apenas porque não encontrou documentação.
- Não substitua o comportamento original por uma interpretação simplificada.
- Não omita funcionalidades secundárias, validações, mensagens, efeitos colaterais ou interações.
- Não limite a análise aos sistemas já conhecidos.
- Descubra funcionalidades adicionais existentes no código, no cliente e nas referências externas.

Toda funcionalidade descoberta deverá ser documentada, mesmo que ainda não exista uma implementação correspondente em Rust.

## 2.2. Não presumir que uma implementação está correta

A existência de uma classe, módulo, função, estrutura ou endpoint em Rust não significa que a funcionalidade esteja implementada corretamente.

É necessário verificar:

- Regras de negócio.
- Validações.
- Entradas e saídas.
- Condições de execução.
- Estados envolvidos.
- Persistência.
- Comunicação com o cliente.
- Eventos produzidos.
- Mensagens enviadas.
- Tratamento de erros.
- Sincronização.
- Efeitos colaterais.
- Comportamento em situações excepcionais.

Uma funcionalidade só poderá ser classificada como portada quando houver evidências suficientes de equivalência comportamental.

## 2.3. Rastreabilidade obrigatória

Toda conclusão técnica deverá possuir referências que permitam verificar sua origem.

Sempre que possível, registrar:

- Repositório.
- Branch.
- Caminho do arquivo.
- Classe, função ou estrutura.
- Número de linha ou intervalo.
- Identificador de protocolo.
- Nome do pacote.
- Referência documental.
- URL da fonte externa.
- Evidência de teste.
- Diferença identificada.
- Conclusão técnica.

Não apresente conclusões sem evidências como se fossem fatos confirmados.

## 2.4. Classificação das informações

Classifique as descobertas utilizando os seguintes níveis:

| Classificação | Significado |
|---|---|
| Confirmado | Comprovado diretamente pelo código, documentação ou teste reproduzível. |
| Fortemente indicado | Existem evidências técnicas consistentes, mas falta confirmação definitiva. |
| Hipótese | Interpretação plausível que ainda precisa ser validada. |
| Desconhecido | Não existem informações suficientes para determinar o comportamento. |
| Divergente | Existem fontes com comportamentos ou informações conflitantes. |
| Não aplicável | A funcionalidade não se aplica à versão ou arquitetura analisada. |

Quando houver divergências entre versões do WYD, não escolha arbitrariamente uma referência. Documente as diferenças e suas possíveis origens.

## 2.5. Não modificar o projeto durante a auditoria sem autorização

A primeira etapa é exclusivamente investigativa.

Não realize refatorações, alterações funcionais, exclusões, migrações ou mudanças estruturais durante a auditoria inicial.

Produza documentação, relatórios, inventários, mapas e planos de implementação.

Caso encontre um problema que exija correção imediata, registre-o como uma descoberta e apresente a proposta de solução.

A implementação de correções deverá acontecer em uma etapa posterior, mediante autorização.

## 2.6. Pesquisa contínua e aprofundamento

Não interrompa a investigação porque encontrou uma implementação aparentemente suficiente.

Investigue dependências, chamadas indiretas, handlers, estruturas compartilhadas, eventos, protocolos e funcionalidades relacionadas.

Sempre que uma descoberta revelar um novo sistema ou uma nova dependência, amplie o escopo da investigação.

Não declare a auditoria completa enquanto existirem áreas relevantes sem análise ou explicitamente classificadas como desconhecidas.

---

# 3. INVENTÁRIO COMPLETO DO ECOSSISTEMA

Antes de comparar funcionalidades, faça um inventário técnico de todo o projeto.

## 3.1. Repositórios e branches

Identifique:

- Repositório OpenWYD.
- Branch `main`.
- Branches relacionadas às implementações em Rust.
- Histórico de commits.
- Tags e versões.
- Módulos compartilhados.
- Bibliotecas externas.
- Dependências.
- Componentes descontinuados.
- Componentes duplicados.
- Implementações alternativas.
- Código experimental.
- Código legado ainda utilizado.

Referência principal do legado:

https://github.com/Jean1dev/w2pp-OpenWYD

Compare essa referência com a branch `main` efetivamente disponível no projeto.

Não presuma que o repositório remoto e a branch local são idênticos.

Registre o commit, a branch e a versão analisada para garantir que a auditoria possa ser reproduzida.

## 3.2. Inventário da implementação Rust

Mapeie toda a estrutura do Rust, identificando:

- Crates.
- Módulos.
- Serviços.
- Sistemas.
- Handlers.
- Managers.
- Repositories.
- Models.
- Entities.
- DTOs.
- Enums.
- Traits.
- Eventos.
- Commands.
- Queries.
- Protocolos.
- Estruturas de pacotes.
- Sistemas de persistência.
- Comunicação de rede.
- Processamento assíncrono.
- Gerenciamento de estado.
- Sistemas de concorrência.
- Testes unitários.
- Testes de integração.
- Testes end-to-end.

Identifique também módulos vazios, implementações provisórias, stubs, mocks utilizados em produção, código duplicado e funcionalidades que parecem concluídas, mas não estão conectadas ao fluxo real do jogo.

## 3.3. Inventário do cliente oficial

Analise integralmente a pasta:

`client/`

Identifique:

- Executáveis.
- Bibliotecas.
- DLLs.
- Arquivos de configuração.
- Arquivos de recursos.
- Arquivos de dados.
- Scripts.
- Estruturas de interface.
- Recursos gráficos.
- Sons.
- Animações.
- Sprites.
- Texturas.
- Meshes.
- Modelos 3D.
- Arquivos de mapas.
- Arquivos de NPCs.
- Arquivos de itens.
- Arquivos de habilidades.
- Arquivos de efeitos.
- Arquivos de fontes.
- Arquivos de localização.
- Arquivos de configuração de interface.
- Bibliotecas de comunicação.
- Possíveis sistemas de proteção e integridade.

Identifique formatos, versões, dependências e ferramentas necessárias para analisar cada tipo de arquivo.

Não limite a investigação aos arquivos com extensões conhecidas. Investigue estruturas binárias, cabeçalhos, assinaturas, formatos proprietários e relacionamentos entre arquivos.

---

# 4. MATRIZ PRINCIPAL DE PARIDADE: OPENWYD × MAIN × RUST

Precisamos criar uma matriz completa de comparação entre as implementações legadas e Rust.

A matriz deverá contemplar, no mínimo, as seguintes colunas:

| Coluna | Descrição |
|---|---|
| ID | Identificador único da funcionalidade. |
| Categoria | Sistema ao qual pertence. |
| Funcionalidade | Nome da funcionalidade ou módulo. |
| Legado OpenWYD | Situação e localização da implementação original. |
| Branch main | Situação da implementação na nossa branch legada. |
| Rust | Situação da implementação em Rust. |
| Cliente | Como o cliente participa da funcionalidade. |
| Protocolo | Pacotes, mensagens e eventos envolvidos. |
| Status | Situação atual da implementação. |
| Paridade | Grau de equivalência comportamental. |
| Descrição | Descrição detalhada do funcionamento. |
| Diferenças | Divergências encontradas entre as implementações. |
| Problemas | Bugs, limitações e comportamentos incorretos. |
| Dependências | Sistemas necessários para seu funcionamento. |
| Evidências | Arquivos, funções, linhas, testes e documentação. |
| Observações | Desafios, particularidades e melhorias futuras. |
| Prioridade | Alta, Média ou Baixa. |
| Complexidade | Baixa, Média, Alta ou Muito Alta. |
| Risco | Risco técnico de implementação ou alteração. |
| Próximas ações | Atividades necessárias para atingir paridade. |

## 4.1. Classificação de status

Utilize estados mais precisos do que apenas implementado ou pendente:

- Não investigado.
- Investigação em andamento.
- Identificado no legado.
- Identificado no cliente.
- Não iniciado em Rust.
- Planejado em Rust.
- Em desenvolvimento.
- Implementado parcialmente.
- Implementado sem integração.
- Integrado, mas não validado.
- Implementado e testado.
- Paridade funcional confirmada.
- Paridade parcial.
- Divergência identificada.
- Bloqueado por dependência.
- Descontinuado na referência analisada.
- Não aplicável.

Não utilize o status "concluído" apenas porque existe código.

## 4.2. Classificação de paridade

| Nível | Descrição |
|---|---|
| P0 | Nenhuma implementação correspondente. |
| P1 | Estrutura inicial ou implementação superficial. |
| P2 | Fluxo principal parcialmente implementado. |
| P3 | Fluxo principal funcional, mas existem divergências. |
| P4 | Funcionalidade equivalente, com exceções ou casos extremos pendentes. |
| P5 | Paridade comportamental comprovada por evidências e testes. |

A paridade deverá considerar o comportamento do cliente, do servidor, dos protocolos e das regras de negócio.

Uma funcionalidade pode ter implementação estrutural completa e ainda apresentar paridade baixa.

---

# 5. AUDITORIA DOS SISTEMAS DO JOGO

Investigue todos os sistemas existentes no legado e no cliente.

Os sistemas abaixo são obrigatórios, mas representam apenas o ponto de partida. Descubra e acrescente todos os demais.

## 5.1. Autenticação e gerenciamento de contas

Investigue:

- Login.
- Logout.
- Autenticação.
- Segunda senha.
- Validação de credenciais.
- Validação de contas.
- Bloqueios.
- Restrições.
- Mensagens de erro.
- Mensagens de sucesso.
- Estados de autenticação.
- Sessões.
- Reconexão.
- Desconexão.
- Expiração de sessão.
- Proteção contra tentativas inválidas.
- Validações do cliente.
- Validações do servidor.
- Fluxos de seleção de servidor e canal, quando existentes.

**Problemas já conhecidos no Rust:**

O legado possui fluxos funcionais, mas Rust ainda apresenta bugs, validações ausentes e tratamento incorreto das mensagens de erro.

Investigue as diferenças e identifique exatamente onde cada comportamento é implementado.

## 5.2. Criação e gerenciamento de personagens

Investigue:

- Criação de personagem.
- Seleção de personagem.
- Exclusão.
- Limites de personagens.
- Classes.
- Atributos.
- Status iniciais.
- Distribuição de pontos.
- Validação de atributos.
- Regras de criação.
- Nomes.
- Restrições.
- Aparência.
- Inventário inicial.
- Equipamentos iniciais.
- Persistência.
- Entrada no mundo.
- Mensagens de erro.
- Regras de CP.
- Atualização dos estados do personagem.

**Problemas já conhecidos no Rust:**

- Problemas nos status iniciais.
- Validações de atributos incorretas ou ausentes.
- Mensagens de erro inconsistentes.
- Problemas relacionados ao CP.
- Divergências em relação à criação de personagens do legado.

Investigue todas as regras e não se limite aos problemas já conhecidos.

## 5.3. Movimentação e mundo

Investigue:

- Movimentação.
- Coordenadas.
- Direções.
- Velocidade.
- Sincronização.
- Colisões.
- Obstáculos.
- Mapas.
- Regiões.
- Teleporte.
- Portais.
- Mudança de mapas.
- Entrada e saída de áreas.
- Atualização da posição.
- Visibilidade de personagens.
- Distâncias.
- Áreas de alcance.
- Caminhos.
- Estados de movimento.
- Interações com objetos.
- Sincronização entre jogadores.
- Mensagens de movimentação.

**Problemas já conhecidos no Rust:**

- Falhas de sincronização.
- Problemas de colisão.
- Mensagens de erro incorretas.
- Possíveis divergências nas regras de movimentação.

Investigue também a relação entre o sistema de mapas, o cliente e o servidor.

## 5.4. NPCs e interações

Investigue:

- NPCs.
- Diálogos.
- Opções de interação.
- Serviços.
- Lojas.
- Teleportes.
- Missões.
- Entrega de itens.
- Recompensas.
- Condições de interação.
- Distância necessária.
- Estados de NPCs.
- Eventos associados.
- Mensagens de interação.
- Scripts e configurações.

**Problemas já conhecidos no Rust:**

- Diálogos incompletos.
- Missões não tratadas corretamente.
- Recompensas inconsistentes.
- Interações que funcionam no legado, mas não possuem equivalência em Rust.

## 5.5. Combate

Investigue:

- Ataques físicos.
- Ataques mágicos.
- Cálculo de dano.
- Defesa.
- Resistências.
- Acerto.
- Esquiva.
- Críticos.
- Habilidades.
- Skills especiais.
- Cooldowns.
- Alcance.
- Área de efeito.
- Buffs.
- Debuffs.
- Efeitos temporários.
- Estados de combate.
- Morte.
- Ressurreição.
- Experiência.
- Drops.
- PVP.
- PVE.
- Combate entre jogadores.
- Combate contra monstros.
- Mensagens de combate.
- Animações e efeitos associados.

**Problemas já conhecidos no Rust:**

- Divergências nos cálculos de dano.
- Habilidades especiais incompletas.
- Mensagens de erro não tratadas corretamente.

Não presuma fórmulas. Reconstrua as regras a partir do código e das referências disponíveis.

## 5.6. Inventário e equipamentos

Investigue:

- Inventário.
- Slots.
- Limites.
- Empilhamento.
- Divisão de quantidades.
- Transferência.
- Movimentação entre slots.
- Equipar.
- Desequipar.
- Remover.
- Descartar.
- Armazenar.
- Retirar.
- Bloqueios.
- Restrições de classe.
- Restrições de nível.
- Durabilidade.
- Aprimoramentos.
- Identificação de itens.
- Itens especiais.
- Itens consumíveis.
- Itens vinculados.
- Itens de missão.
- Persistência.
- Sincronização com o cliente.

## 5.7. Sistema de itens

Investigue integralmente:

- Tipos de itens.
- Categorias.
- Identificadores.
- Propriedades.
- Atributos.
- Efeitos.
- Requisitos.
- Classes permitidas.
- Níveis mínimos.
- Durabilidade.
- Encantamentos.
- Aprimoramentos.
- Combinações.
- Consumo.
- Utilização.
- Transferência.
- Venda.
- Compra.
- Descarte.
- Entrega em NPC.
- Utilização por clique.
- Teleporte.
- Ativação de eventos.
- Execução de ações.
- Recompensas.
- Efeitos visuais.
- Animações.
- Sons.
- Representação gráfica.

Mapeie a diferença entre a definição de um item, seu comportamento no servidor e sua representação visual no cliente.

## 5.8. Comércio e economia

Investigue:

- Lojas.
- Compra.
- Venda.
- Preços.
- Taxas.
- Moedas.
- Transações.
- Trocas entre jogadores.
- Comércio privado.
- Armazenamento.
- Bancos.
- Leilões, caso existam.
- Validação de saldo.
- Sincronização de inventário.
- Concorrência.
- Prevenção de duplicação.
- Reversão de transações.
- Persistência.
- Histórico de operações.

**Problemas já conhecidos no Rust:**

- Falhas de sincronização de inventário.
- Divergências de preços.
- Tratamento inadequado de mensagens de erro.
- Inconsistências em transações.

Investigue também a economia completa do jogo, mesmo que ainda não exista uma visão consolidada de todas as regras.

## 5.9. Chat e comunicação

Investigue:

- Chat geral.
- Chat privado.
- Mensagens entre jogadores.
- Canais.
- Mensagens de sistema.
- Mensagens de NPCs.
- Mensagens de eventos.
- Filtros.
- Restrições.
- Bloqueios.
- Formatação.
- Limites.
- Sincronização.
- Entrega.
- Recebimento.
- Notificações.
- Mensagens de erro.

**Problemas já conhecidos no Rust:**

Falhas no envio e recebimento de mensagens, filtros e tratamento de erros.

## 5.10. Missões e quests

O legado possui implementação parcial, com a maioria das funcionalidades aparentemente operacionais, mas ainda existem bugs.

No Rust, o sistema ainda não foi implementado.

Faça uma investigação específica e detalhada:

- Estrutura das missões.
- Identificadores.
- Tipos de missão.
- Pré-requisitos.
- Aceitação.
- Recusa.
- Abandono.
- Progresso.
- Objetivos.
- Contadores.
- Eventos de progresso.
- NPCs envolvidos.
- Itens necessários.
- Monstros necessários.
- Recompensas.
- Conclusão.
- Repetição.
- Limites.
- Missões diárias, caso existam.
- Missões encadeadas.
- Missões temporárias.
- Persistência.
- Estados.
- Comunicação com o cliente.
- Mensagens.
- Interfaces.
- Indicadores visuais.

Mapeie todas as missões encontradas, não apenas o motor principal.

## 5.11. Eventos

Investigue:

- Eventos automáticos.
- Eventos agendados.
- Eventos manuais.
- Eventos sazonais.
- Eventos de mapas.
- Eventos de combate.
- Eventos de NPCs.
- Eventos globais.
- Ativação.
- Desativação.
- Participação.
- Condições.
- Recompensas.
- Sincronização.
- Persistência.
- Mensagens.
- Estados.
- Temporizadores.

**Problemas já conhecidos no Rust:**

Problemas de sincronização, ativação e desativação, além do tratamento inadequado de mensagens.

## 5.12. Guildas

Investigue:

- Criação.
- Convites.
- Aceitação.
- Recusa.
- Exclusão.
- Gerenciamento.
- Hierarquia.
- Cargos.
- Permissões.
- Comunicação.
- Membros.
- Liderança.
- Regras de participação.
- Conflitos.
- Eventos.
- Guerras.
- Benefícios.
- Sistemas relacionados.
- Persistência.

**Problemas já conhecidos no Rust:**

Problemas de criação, gerenciamento, comunicação entre membros e tratamento de erros.

## 5.13. Cidadania

Investigue:

- Atribuição.
- Remoção.
- Requisitos.
- Permissões.
- Restrições.
- Benefícios.
- Relações com cidades.
- Relações com guildas.
- Relações com guerras.
- Relações com NPCs.
- Estados.
- Persistência.
- Comunicação com o cliente.

O funcionamento completo ainda não está dimensionado. Descubra todas as regras existentes no legado e nas referências.

## 5.14. Demais sistemas

Não encerre a investigação nos sistemas descritos acima.

Descubra todos os sistemas adicionais encontrados no projeto, incluindo, quando existentes:

- Progressão de personagem.
- Experiência e níveis.
- Distribuição de pontos.
- Classes e evoluções.
- Skills e árvores de habilidades.
- Montarias.
- Companheiros.
- Mascotes.
- Sistemas de aprimoramento.
- Sistemas de combinação.
- Refinação.
- Encantamento.
- Drops.
- Monstros.
- Chefes.
- Respawn.
- Mapas e instâncias.
- Dungeons.
- Guerras.
- Castle Siege.
- Sistemas de ranking.
- Sistemas de reputação.
- Sistemas de PK.
- Sistemas de punição.
- Sistemas administrativos.
- Comandos GM.
- Logs.
- Auditoria.
- Sistemas de segurança.
- Anti-cheat.
- Configurações globais.
- Sistemas de persistência.
- Backup.
- Recuperação.
- Comunicação entre serviços.
- Sistemas de inicialização.
- Gerenciamento de conexões.
- Sincronização de estado.

Essa lista não é limitadora. Qualquer funcionalidade encontrada deverá ser registrada.

---

# 6. AUDITORIA ESPECÍFICA DO CLIENTE OFICIAL — PASTA client/

Esta é uma das partes mais importantes do trabalho.

Não queremos analisar o cliente apenas como um conjunto de arquivos gráficos. Precisamos compreender seu funcionamento interno e sua integração com o servidor.

## 6.1. Matriz de funcionalidades do cliente

Crie uma matriz específica para o cliente contendo:

| Coluna | Descrição |
|---|---|
| ID | Identificador da funcionalidade. |
| Sistema | Sistema do jogo relacionado. |
| Funcionalidade | Recurso existente no cliente. |
| Arquivo | Executável, biblioteca, asset ou arquivo relacionado. |
| Localização | Caminho dentro de `client/`. |
| Componente | Módulo, função, estrutura ou recurso identificado. |
| Comunicação | Como o cliente interage com o servidor. |
| Pacotes enviados | Mensagens enviadas ao servidor. |
| Pacotes recebidos | Mensagens recebidas do servidor. |
| Eventos | Eventos internos relacionados. |
| Legado | Como o servidor legado trata a funcionalidade. |
| Rust | Como a implementação Rust trata a funcionalidade. |
| Status | Situação atual. |
| Diferenças | Divergências encontradas. |
| Evidências | Referências técnicas. |
| Melhorias | Possibilidades de customização. |

## 6.2. Fluxos completos de comunicação

Para cada funcionalidade, documente o fluxo completo:

1. Ação realizada pelo jogador.
2. Evento gerado pelo cliente.
3. Validações locais.
4. Estrutura de dados preparada.
5. Pacote construído.
6. Serialização.
7. Envio pela conexão.
8. Recepção pelo servidor.
9. Identificação do pacote.
10. Desserialização.
11. Validações do servidor.
12. Execução da regra de negócio.
13. Alteração de estado.
14. Persistência, quando aplicável.
15. Resposta do servidor.
16. Recepção no cliente.
17. Atualização da interface.
18. Atualização visual ou sonora.
19. Tratamento de erros.
20. Comportamento em situações excepcionais.

Nem todos os fluxos terão todos esses estágios. Documente os efetivamente existentes e identifique os que não foram descobertos.

## 6.3. Identificação de funcionalidades não portadas

O cliente possui diversas funcionalidades que já são suportadas pelo legado, mas ainda não foram implementadas ou estão incompletas em Rust.

Precisamos descobrir:

- Quais recursos o cliente oferece.
- Quais mensagens ele envia.
- Quais respostas espera receber.
- Quais funcionalidades estão disponíveis no legado.
- Quais funcionalidades ainda não existem em Rust.
- Quais funcionalidades estão parcialmente implementadas.
- Quais funcionalidades possuem protocolos incompatíveis.
- Quais comportamentos dependem de informações do cliente.
- Quais recursos exigem modificações no cliente.

Não considere que uma funcionalidade não existe apenas porque não encontrou uma função com o mesmo nome.

Investigue também implementações indiretas e compartilhadas.

---

# 7. ENGENHARIA REVERSA DO CLIENTE WYD2

## 7.1. Objetivo

Historicamente, servidores privados de WYD2 realizavam modificações no cliente por meio de ferramentas como OllyDbg, alterações em binários, hooks e modificações em memória.

Essas técnicas permitiam modificar:

- Teleportes.
- Interfaces.
- Comportamentos.
- Funções.
- Endereços.
- Hooks.
- Recursos visuais.
- Limitações do cliente.
- Comunicação.
- Funcionalidades adicionais.

Atualmente, precisamos avaliar quais dessas técnicas continuam viáveis e quais ferramentas e abordagens modernas oferecem resultados melhores.

Não temos acesso confirmado ao código-fonte original completo do cliente.

Precisamos investigar se existem fontes públicas, projetos alternativos, implementações independentes ou materiais técnicos que possam facilitar o desenvolvimento.

## 7.2. Pesquisa de código-fonte

Investigue:

- Repositórios públicos do cliente WYD2.
- Clientes alternativos.
- Projetos open source.
- Forks.
- Projetos abandonados.
- Implementações de clientes em outras linguagens.
- Bibliotecas relacionadas.
- Documentações técnicas.
- Fóruns especializados.
- Comunidades de desenvolvimento.
- Projetos de engenharia reversa.
- Ferramentas de modificação.
- Documentações sobre formatos proprietários.

Pesquise também por referências históricas e nomes alternativos do WYD, inclusive quando os projetos não utilizarem explicitamente o nome OpenWYD.

Para cada repositório encontrado, avalie:

- Autenticidade.
- Origem.
- Linguagem.
- Estado de manutenção.
- Funcionalidades.
- Compatibilidade.
- Utilidade como referência.
- Qualidade do código.
- Licença.
- Restrições de uso.
- Possibilidade de reutilização.

Não confunda implementações independentes com o código-fonte oficial.

## 7.3. Engenharia reversa de executáveis

Investigue ferramentas e metodologias para:

- Análise estática.
- Análise dinâmica.
- Desmontagem.
- Descompilação.
- Análise de assembly.
- Identificação de funções.
- Identificação de strings.
- Identificação de referências.
- Identificação de estruturas.
- Análise de chamadas.
- Análise de memória.
- Identificação de bibliotecas.
- Análise de dependências.
- Análise de formatos binários.
- Identificação de funções relacionadas a protocolos.
- Identificação de recursos embutidos.
- Identificação de sistemas gráficos.

Avalie ferramentas como:

- Ghidra.
- IDA Free / IDA Pro.
- Binary Ninja.
- x64dbg.
- OllyDbg.
- WinDbg.
- Cheat Engine para análise autorizada de memória.
- Detect It Easy.
- PE-bear.
- CFF Explorer.
- dnSpy / ILSpy, caso existam componentes .NET.
- Radare2 / Rizin.
- Frida.
- Process Monitor.
- Process Explorer.
- Wireshark.

Não assuma que todas são compatíveis com a arquitetura do cliente. Identifique se o executável é 32 ou 64 bits, sua arquitetura, dependências, compilador provável e características do ambiente de execução.

Compare as ferramentas e recomende uma combinação adequada para nosso cenário.

## 7.4. Análise dinâmica e comportamento em execução

Investigue possibilidades de:

- Identificação de funções executadas.
- Rastreamento de chamadas.
- Monitoramento de memória.
- Inspeção de estruturas.
- Monitoramento de arquivos.
- Monitoramento de bibliotecas.
- Identificação de eventos.
- Análise de estados.
- Rastreamento de comunicação.
- Identificação de recursos carregados.
- Identificação de chamadas de sistema.
- Observação de comportamentos durante interações do jogador.

O objetivo é compreender como o cliente funciona e identificar os pontos de extensão e customização.

## 7.5. Engenharia reversa de protocolos e pacotes

Investigue como identificar e documentar:

- Protocolos.
- Cabeçalhos.
- Identificadores.
- Tamanhos.
- Campos.
- Tipos.
- Flags.
- Checksums.
- Criptografia.
- Compactação.
- Serialização.
- Desserialização.
- Endianness.
- Sequenciamento.
- Fragmentação.
- Mensagens.
- Eventos.
- Estados.
- Respostas.
- Erros.
- Reconexões.

Avalie:

- Wireshark.
- Capturas PCAP.
- Análise de tráfego em ambiente controlado.
- Instrumentação de processos.
- Instrumentação de funções de rede.
- Comparação de pacotes antes e depois de ações específicas.
- Análise de buffers.
- Comparação entre comunicação do legado e Rust.

Para cada pacote identificado, documente:

| Campo | Descrição |
|---|---|
| ID | Identificador do pacote. |
| Nome | Nome funcional. |
| Direção | Cliente → servidor ou servidor → cliente. |
| Tamanho | Tamanho fixo, variável ou desconhecido. |
| Cabeçalho | Estrutura identificada. |
| Campos | Campos e respectivos tipos. |
| Serialização | Forma de codificação. |
| Validações | Regras aplicadas. |
| Handler legado | Localização no OpenWYD. |
| Handler Rust | Localização em Rust. |
| Cliente | Comportamento associado. |
| Resposta | Resposta esperada. |
| Erros | Erros possíveis. |
| Evidências | Referências e capturas. |
| Confiança | Nível de confirmação. |

Não invente estruturas de pacotes. Quando um campo não puder ser identificado, registre como desconhecido.

## 7.6. Estratégia de engenharia reversa

Compare três abordagens:

**Abordagem A — Engenharia reversa do executável**

Análise estática, dinâmica, assembly, memória e chamadas.

**Abordagem B — Análise de comunicação**

Captura de pacotes, instrumentação, reconstrução de protocolos e comparação entre cliente e servidor.

**Abordagem C — Análise dos recursos e arquivos**

Investigação dos formatos gráficos, configurações, dados, estruturas binárias e assets.

Avalie também uma quarta abordagem:

**Abordagem D — Desenvolvimento de cliente independente**

Investigue a viabilidade de criar um cliente próprio, utilizando os recursos gráficos, modelos, sprites, animações, sons e mapas existentes, quando tecnicamente e legalmente permitido.

Para cada abordagem, apresente:

- Viabilidade.
- Dificuldade.
- Ferramentas.
- Conhecimento necessário.
- Limitações.
- Riscos.
- Benefícios.
- Custos de manutenção.
- Dependências.
- Resultados esperados.
- Aplicabilidade ao projeto.

Ao final, recomende uma estratégia combinada.

### Observação jurídica e de segurança

O projeto tem finalidade de estudo, aprendizado e desenvolvimento interno, sem intenção comercial ou distribuição pública dos materiais proprietários.

Isso não significa que direitos autorais, licenças, contratos ou restrições legais deixem automaticamente de existir.

Priorize engenharia reversa de materiais próprios, autorizados ou cujo uso seja legalmente permitido. Avalie as condições jurídicas aplicáveis antes de reutilizar, distribuir ou modificar componentes proprietários.

Não transforme a investigação em instruções para comprometer sistemas de terceiros, roubar credenciais ou acessar infraestrutura não autorizada.

---

# 8. CUSTOMIZAÇÃO VISUAL E MODIFICAÇÃO DO CLIENTE

Queremos investigar tudo o que pode ser efetivamente modificado no cliente.

Não estamos limitados à implementação dos sistemas oficiais. Após compreender a arquitetura, queremos explorar possibilidades de personalização visual e funcional.

## 8.1. Interface e identidade visual

Investigue como editar:

- Tela inicial.
- Tela de login.
- Seleção de personagens.
- Interface principal.
- HUD.
- Barras de status.
- Janelas.
- Menus.
- Botões.
- Ícones.
- Inventário.
- Skills.
- Mapas.
- Minimap.
- Chat.
- Interfaces de NPCs.
- Interfaces de eventos.
- Telas de carregamento.
- Fontes.
- Cores.
- Texturas.
- Fundos.
- Animações de interface.
- Elementos visuais de interação.

Identifique quais recursos estão armazenados externamente, quais estão embutidos no executável e quais dependem de funções internas.

## 8.2. Sprites, texturas e imagens

Investigue:

- Formatos.
- Compressão.
- Resolução.
- Transparência.
- Paletas.
- Animações.
- Sequências de frames.
- Coordenadas.
- Referências internas.
- Relação com objetos do jogo.
- Ferramentas de edição.
- Ferramentas de conversão.
- Possibilidades de substituição.

Identifique o fluxo necessário para criar, editar, importar, testar e utilizar novos recursos.

## 8.3. Meshes e modelos 3D

Investigue:

- Formatos de mesh.
- Geometria.
- Vértices.
- Faces.
- Normais.
- UV mapping.
- Materiais.
- Texturas.
- Skeletons.
- Bones.
- Rigging.
- Animações.
- Hierarquia de objetos.
- Escalas.
- Coordenadas.
- Compatibilidade.
- Conversão.
- Exportação.
- Importação.

Pesquise ferramentas adequadas, incluindo Blender e possíveis plugins, scripts e conversores específicos para formatos proprietários do WYD.

Determine se é possível utilizar ferramentas modernas para criar novos modelos e convertê-los para os formatos reconhecidos pelo cliente.

## 8.4. Animações

Investigue:

- Animações de personagens.
- Animações de monstros.
- Animações de NPCs.
- Animações de armas.
- Animações de combate.
- Animações de skills.
- Animações de movimentos.
- Animações de morte.
- Animações de interação.
- Efeitos especiais.
- Partículas.
- Efeitos visuais.
- Sequências de frames.
- Timings.
- Transições.
- Skeletons.
- Compatibilidade entre modelos.

Documente como criar novas animações e modificar as existentes.

## 8.5. Mapas e ambientes

Investigue:

- Estruturas de mapas.
- Terrenos.
- Texturas.
- Objetos.
- Obstáculos.
- Colisões.
- Coordenadas.
- Áreas navegáveis.
- Regiões.
- Portais.
- Pontos de spawn.
- NPCs.
- Elementos ambientais.
- Efeitos.
- Iluminação.
- Recursos de ambiente.

Identifique como editar mapas, criar novas áreas e manter a compatibilidade com as regras de movimentação e colisão do servidor.

---

# 9. CRIAÇÃO E CUSTOMIZAÇÃO DE ITENS, ARMAS E SETS

Essa área deverá receber uma documentação específica, pois queremos ampliar o conteúdo do jogo futuramente.

## 9.1. Anatomia completa de um item

Descubra todas as estruturas envolvidas na criação de um item:

- ID.
- Nome.
- Tipo.
- Categoria.
- Subcategoria.
- Atributos.
- Status.
- Requisitos.
- Classe.
- Nível.
- Peso.
- Durabilidade.
- Valor.
- Preço.
- Efeitos.
- Propriedades especiais.
- Ícone.
- Sprite.
- Mesh.
- Textura.
- Animação.
- Som.
- Comportamento no servidor.
- Comportamento no cliente.
- Persistência.
- Interações.

Identifique quais propriedades são definidas em arquivos de dados e quais são controladas diretamente pelo código.

## 9.2. Itens funcionais

Precisamos entender como criar itens com funcionalidades próprias.

Exemplos:

- Item utilizável por clique.
- Item que pode ser entregue a um NPC.
- Item que ativa um teleporte.
- Item que concede recompensas.
- Item que executa uma ação.
- Item que inicia uma missão.
- Item que participa de uma combinação.
- Item consumível.
- Item com efeito temporário.
- Item com efeito permanente.
- Item que ativa um evento.
- Item com restrições específicas.

Para cada tipo, documente o fluxo completo, desde a definição do item até a execução da funcionalidade no servidor e sua representação no cliente.

## 9.3. Criação de armas

Investigue:

- Tipos de armas.
- Modelos.
- Meshes.
- Texturas.
- Ícones.
- Animações.
- Sons.
- Atributos.
- Dano.
- Velocidade.
- Alcance.
- Requisitos.
- Classes.
- Efeitos.
- Skills associadas.
- Comportamento no combate.
- Compatibilidade com personagens.

## 9.4. Criação de novos sets

Investigue:

- Estrutura dos sets existentes.
- Peças.
- Identificadores.
- Requisitos.
- Bônus individuais.
- Bônus de conjunto.
- Quantidade de peças.
- Efeitos visuais.
- Modelos.
- Texturas.
- Compatibilidade.
- Regras de ativação.
- Representação no cliente.
- Processamento no servidor.

## 9.5. Expansão de conteúdo

Documente como criar novos:

- Itens.
- Armas.
- Armaduras.
- Sets.
- Acessórios.
- Consumíveis.
- NPCs.
- Monstros.
- Bosses.
- Habilidades.
- Efeitos.
- Animações.
- Mapas.
- Eventos.
- Missões.

Essa documentação deverá servir como um manual técnico para futuras expansões do WYD2.

---

# 10. DOCUMENTAÇÃO EXISTENTE — PASTA docs/

A pasta `docs/` contém conhecimento já produzido sobre o funcionamento do jogo, sistemas, itens e outras funcionalidades.

Ela deverá ser tratada como uma das principais fontes internas de investigação.

## 10.1. Auditoria documental

Faça um inventário de todos os documentos.

Para cada documento, identifique:

- Título.
- Assunto.
- Sistemas relacionados.
- Informações relevantes.
- Referências de código.
- Versão do jogo.
- Origem das informações.
- Data, quando disponível.
- Informações confirmadas.
- Informações incompletas.
- Possíveis erros.
- Contradições.
- Documentação desatualizada.

## 10.2. Validação documental

Compare o conteúdo da documentação com:

1. OpenWYD original.
2. Branch `main`.
3. Implementação Rust.
4. Cliente oficial.
5. Referências externas.

Não considere a documentação existente automaticamente correta.

Confirme suas informações no código e em evidências reais.

Quando encontrar divergências, preserve o conteúdo original como histórico e documente a correção ou atualização em uma seção apropriada.

## 10.3. Expansão da base de conhecimento

Toda descoberta relevante deverá ser incorporada à documentação.

Não deixe o conhecimento apenas no relatório final ou na conversa com o agente.

A documentação deverá permitir que outro desenvolvedor ou agente de IA compreenda o funcionamento do sistema sem precisar repetir toda a investigação.

---

# 11. PESQUISA EXTERNA — REFERÊNCIAS DO WYD

Realize pesquisas profundas na internet.

Utilize repositórios, sites oficiais, documentação, fóruns, comunidades, servidores históricos, vídeos técnicos, manuais e materiais de referência.

Entretanto, as fontes deverão possuir uma hierarquia de confiabilidade.

## 11.1. Referência 1 — WYD2 Ongame Brasil

O WYD2 oficial brasileiro, distribuído pela JoyImpact por meio da Ongame, é uma referência histórica importante.

Características:

- Versão oficial.
- Algumas funcionalidades estavam menos atualizadas em comparação com versões internacionais.
- Possui comportamentos históricos relevantes.
- Sistemas existentes nessa versão devem ser considerados referências válidas para o contexto brasileiro.

Utilize essa versão para identificar comportamentos oficiais e particularidades históricas.

## 11.2. Referência 2 — WYD Internacional

O WYD Internacional foi durante muito tempo uma das versões mais atualizadas do jogo, mas foi descontinuado.

Características:

- Continuidade de sistemas do WYD.
- Funcionalidades adicionais.
- Evolução de mecânicas.
- Correções.
- Modificações de sistemas.
- Referências históricas importantes.

Tudo o que for encontrado deverá ser analisado como referência válida, respeitando as diferenças entre versões.

## 11.3. Referência 3 — WYD Global

O WYD Global é atualmente uma das referências mais importantes para evolução dos sistemas.

Características:

- Continuidade do WYD Internacional.
- Continuidade dos sistemas do WYD Ongame.
- Sistemas mais recentes.
- Atualizações.
- Melhorias.
- Correções.
- Novas funcionalidades.
- Manuais.
- Páginas oficiais.
- Informações sobre sistemas.

Temos acesso ao site oficial e podemos utilizar páginas, manuais e materiais públicos para compreender seu funcionamento.

Quando houver diferenças entre as versões, registre-as.

Não substitua automaticamente uma regra histórica do WYD2 brasileiro por uma regra do Global.

## 11.4. Referência 4 — Outros servidores WYD

Pesquise servidores privados e implementações independentes.

Esses servidores podem apresentar:

- Sistemas modificados.
- Mecânicas facilitadas.
- Novos itens.
- Novos mapas.
- Alterações de atributos.
- Ajustes econômicos.
- Eventos.
- Novas funcionalidades.
- Correções.
- Modificações no cliente.
- Extensões do protocolo.

Não considere essas implementações referências oficiais.

Utilize-as como fontes de inspiração e conhecimento técnico.

Quando encontrar uma funcionalidade diferente, documente:

- Como funciona.
- Onde foi encontrada.
- Qual versão ou servidor apresenta o comportamento.
- Qual a diferença em relação às referências oficiais.
- Se pode ser útil para nosso projeto.
- Quais vantagens e desvantagens apresenta.

Não implemente automaticamente essas diferenças. Registre-as como propostas para avaliação posterior.

## 11.5. Hierarquia de referências

Utilize a seguinte ordem de preferência:

1. Comportamento comprovado no cliente e no legado que estamos desenvolvendo.
2. Implementação oficial do WYD2 Ongame.
3. Referências do WYD Internacional.
4. Referências atuais do WYD Global.
5. Documentação oficial e materiais técnicos.
6. Repositórios públicos independentes.
7. Fóruns e comunidades.
8. Servidores privados.
9. Hipóteses e interpretações.

Essa hierarquia não significa que uma versão mais antiga esteja sempre correta para o nosso projeto.

A versão-alvo e a compatibilidade desejada devem ser consideradas em cada funcionalidade.

## 11.6. Registro das pesquisas

Toda informação externa relevante deverá possuir:

- URL.
- Título.
- Autor ou origem, quando disponível.
- Data de publicação ou atualização.
- Versão do jogo relacionada.
- Data da consulta.
- Informação extraída.
- Confiabilidade.
- Comparação com o código local.
- Divergências identificadas.

Não utilize informações de fóruns como verdades absolutas.

Cruze fontes sempre que possível.

---

# 12. ANÁLISE DE ARQUITETURA E QUALIDADE DO RUST

Além da paridade funcional, precisamos avaliar a qualidade técnica da implementação Rust.

## 12.1. Arquitetura

Investigue:

- Organização dos crates.
- Separação de responsabilidades.
- Acoplamento.
- Coesão.
- Dependências.
- Interfaces.
- Abstrações.
- Modularidade.
- Gerenciamento de estado.
- Comunicação entre módulos.
- Persistência.
- Tratamento de erros.
- Concorrência.
- Processamento assíncrono.

## 12.2. Práticas de Rust

Avalie:

- Ownership.
- Borrowing.
- Lifetimes.
- Traits.
- Generics.
- Enums.
- Pattern matching.
- Error handling.
- Result e Option.
- Async/await.
- Tokio, quando utilizado.
- Segurança de memória.
- Uso de Arc e Mutex.
- Concorrência.
- Performance.
- Alocação.
- Cópias desnecessárias.
- Uso de unsafe.
- Panics.
- Tratamento de desconexões.

Identifique problemas de arquitetura e implementação que possam comprometer a estabilidade do servidor.

## 12.3. Qualidade de código

Avalie:

- Duplicação.
- Complexidade.
- Funções extensas.
- Código morto.
- TODOs.
- Implementações provisórias.
- Hardcoding.
- Tratamento de erros genérico.
- Logs insuficientes.
- Ausência de testes.
- Cobertura.
- Dependências desnecessárias.
- Problemas de performance.
- Inconsistências entre módulos.

## 12.4. Persistência e integridade

Investigue:

- Modelagem dos dados.
- Estruturas do banco.
- Relacionamentos.
- Transações.
- Consistência.
- Concorrência.
- Bloqueios.
- Atualizações simultâneas.
- Recuperação após falhas.
- Salvamento de estados.
- Cache.
- Sincronização.
- Integridade de inventário.
- Integridade de transações.
- Proteção contra duplicação de operações.

## 12.5. Segurança e confiabilidade

Investigue:

- Validação de entrada.
- Confiança indevida no cliente.
- Manipulação de pacotes.
- Pacotes malformados.
- Repetição de operações.
- Corridas de estado.
- Abuso de funcionalidades.
- Exploração de duplicação de itens.
- Integridade de transações.
- Proteção de sessões.
- Limites de conexão.
- Rate limiting.
- Isolamento de jogadores.
- Falhas de autorização.

Identifique também as diferenças entre as validações realizadas pelo legado e aquelas existentes no Rust.

---

# 13. TESTES DE PARIDADE E REGRESSÃO

A análise não deverá se limitar à leitura do código.

Precisamos criar uma estratégia para comprovar que Rust reproduz corretamente o comportamento esperado.

## 13.1. Testes comparativos

Sempre que possível, execute os mesmos cenários no legado e em Rust.

Compare:

- Entradas.
- Saídas.
- Estados.
- Mensagens.
- Pacotes.
- Alterações no banco.
- Eventos.
- Resultados.
- Erros.
- Efeitos colaterais.

## 13.2. Testes de protocolo

Crie uma estratégia de testes para verificar:

- Serialização.
- Desserialização.
- Compatibilidade de pacotes.
- Campos obrigatórios.
- Tamanhos.
- Valores limites.
- Pacotes inválidos.
- Mensagens inesperadas.
- Respostas.
- Sequenciamento.
- Reconexão.

## 13.3. Testes funcionais

Para cada sistema, documente:

- Cenário normal.
- Cenário de erro.
- Cenário de limite.
- Cenário de concorrência.
- Cenário de desconexão.
- Cenário de recuperação.
- Cenário de abuso.
- Cenário de integração.

## 13.4. Testes de regressão

Toda correção futura deverá possuir testes que impeçam o retorno de problemas já solucionados.

Avalie a possibilidade de criar:

- Testes unitários.
- Testes de integração.
- Testes end-to-end.
- Testes de protocolo.
- Testes comparativos.
- Testes de carga.
- Testes de stress.
- Testes de estabilidade.

Não invente resultados de testes que não foram executados.

---

# 14. PRIORIZAÇÃO E ROADMAP DE IMPLEMENTAÇÃO

Depois de concluir o levantamento, organize todas as descobertas em um roadmap.

## 14.1. Prioridades

**Alta:**

- Funcionalidades essenciais para jogar.
- Falhas de autenticação.
- Problemas de criação de personagens.
- Movimentação.
- Combate.
- Inventário.
- Persistência.
- Comunicação cliente-servidor.
- Bugs que comprometem a integridade dos dados.
- Funcionalidades necessárias para outros sistemas.

**Média:**

- Sistemas importantes, mas não bloqueadores para o funcionamento básico.
- Missões.
- Eventos.
- Guildas.
- Economia.
- Cidadania.
- Funcionalidades secundárias.
- Melhorias de arquitetura.

**Baixa:**

- Melhorias visuais.
- Customizações.
- Novos conteúdos.
- Novos itens.
- Novos sets.
- Novas animações.
- Expansões não essenciais.

A prioridade inicial poderá ser revisada quando as dependências forem identificadas.

## 14.2. Critérios de priorização

Considere:

- Importância para o jogo.
- Dependências.
- Impacto funcional.
- Quantidade de jogadores afetados.
- Complexidade.
- Risco.
- Esforço estimado.
- Possibilidade de reutilização.
- Bloqueios.
- Necessidade de alterações no cliente.
- Necessidade de mudanças no protocolo.

## 14.3. Roadmap

Organize as tarefas por:

- Fase.
- Sistema.
- Funcionalidade.
- Dependências.
- Prioridade.
- Complexidade.
- Arquivos envolvidos.
- Entregáveis.
- Critérios de aceite.
- Testes necessários.
- Riscos.

Separe claramente:

1. Correções de bugs.
2. Implementação de funcionalidades ausentes.
3. Adequação de protocolos.
4. Melhorias arquiteturais.
5. Otimizações.
6. Customização do cliente.
7. Expansão de conteúdo.

Não misture a implementação das funcionalidades oficiais com a criação de sistemas customizados.

Primeiro estabeleça a paridade funcional da versão-alvo. Depois organize as expansões.

---

# 15. ORGANIZAÇÃO DOS ENTREGÁVEIS E DOCUMENTAÇÃO

Toda a investigação deverá gerar documentação persistente dentro do projeto.

Antes de criar arquivos, examine a estrutura existente de `docs/` e reutilize os padrões já estabelecidos, evitando duplicações.

Caso não exista uma organização adequada, proponha uma estrutura semelhante à seguinte:

```text
docs/
└── audit/
    ├── README.md
    ├── 00-executive-summary.md
    ├── 01-project-inventory.md
    ├── 02-legacy-analysis.md
    ├── 03-rust-analysis.md
    ├── 04-client-analysis.md
    ├── 05-feature-parity-matrix.md
    ├── 06-protocol-catalog.md
    ├── 07-network-flows.md
    ├── 08-reverse-engineering.md
    ├── 09-client-customization.md
    ├── 10-assets-and-formats.md
    ├── 11-items-and-equipment.md
    ├── 12-maps-and-animations.md
    ├── 13-external-references.md
    ├── 14-rust-architecture-audit.md
    ├── 15-bugs-and-divergences.md
    ├── 16-test-strategy.md
    ├── 17-priorities-and-roadmap.md
    ├── 18-unknown-features.md
    └── systems/
        ├── authentication.md
        ├── characters.md
        ├── movement.md
        ├── combat.md
        ├── inventory.md
        ├── items.md
        ├── npcs.md
        ├── quests.md
        ├── events.md
        ├── guilds.md
        ├── economy.md
        ├── citizenship.md
        └── additional-systems.md
```

Essa estrutura é uma sugestão inicial. Adapte-a à organização existente e aos sistemas efetivamente encontrados.

## 15.1. Entregáveis obrigatórios

Ao final da auditoria, precisamos ter:

1. Inventário completo do projeto.
2. Inventário da implementação Rust.
3. Inventário do legado.
4. Inventário do cliente.
5. Matriz geral de paridade.
6. Matriz específica do cliente.
7. Catálogo de protocolos.
8. Mapeamento dos fluxos de comunicação.
9. Mapeamento de eventos.
10. Catálogo de funcionalidades.
11. Catálogo de itens.
12. Mapeamento de recursos gráficos.
13. Relatório de engenharia reversa.
14. Pesquisa de clientes e códigos-fonte disponíveis.
15. Comparação das ferramentas de engenharia reversa.
16. Documentação das possibilidades de customização.
17. Auditoria de arquitetura Rust.
18. Relatório de bugs e divergências.
19. Estratégia de testes.
20. Roadmap de implementação.
21. Catálogo de dúvidas e funcionalidades desconhecidas.
22. Relatório executivo de conclusão.

## 15.2. Diagramas

Quando útil, produza diagramas Mermaid para documentar:

- Arquitetura.
- Comunicação cliente-servidor.
- Fluxos de autenticação.
- Fluxos de combate.
- Fluxos de inventário.
- Fluxos de comércio.
- Fluxos de missões.
- Dependências entre sistemas.
- Processamento de pacotes.
- Eventos.
- Persistência.
- Roadmap.

Os diagramas devem representar comportamentos comprovados ou indicar explicitamente as partes hipotéticas.

---

# 16. METODOLOGIA DE EXECUÇÃO

A investigação deverá acontecer em fases.

## FASE 1 — Reconhecimento do projeto

- Identificar diretórios.
- Identificar repositórios.
- Identificar branches.
- Identificar versões.
- Identificar estrutura Rust.
- Identificar legado.
- Identificar cliente.
- Identificar documentação.
- Identificar ferramentas disponíveis.

**Entregável:** inventário técnico inicial.

## FASE 2 — Auditoria do legado

- Mapear sistemas.
- Identificar regras de negócio.
- Identificar handlers.
- Identificar protocolos.
- Identificar mensagens.
- Identificar persistência.
- Identificar eventos.
- Identificar dependências.
- Identificar comportamentos e bugs.

**Entregável:** mapa funcional do OpenWYD e da branch `main`.

## FASE 3 — Auditoria Rust

- Mapear implementações.
- Analisar fluxos.
- Identificar integrações.
- Identificar funcionalidades incompletas.
- Identificar divergências.
- Avaliar arquitetura.
- Avaliar testes.
- Identificar riscos.

**Entregável:** mapa funcional e técnico do Rust.

## FASE 4 — Auditoria do cliente

- Analisar arquivos.
- Identificar funcionalidades.
- Identificar recursos.
- Mapear comunicação.
- Identificar protocolos.
- Mapear eventos.
- Identificar funcionalidades não descobertas.

**Entregável:** inventário e matriz do cliente.

## FASE 5 — Engenharia reversa e pesquisa externa

- Pesquisar fontes.
- Pesquisar ferramentas.
- Avaliar executáveis.
- Investigar protocolos.
- Investigar recursos.
- Avaliar customizações.
- Investigar modelos e formatos.
- Identificar possibilidades de criação de conteúdo.

**Entregável:** relatório de engenharia reversa e customização.

## FASE 6 — Comparação e paridade

- Cruzar as três bases: legado, cliente e Rust.
- Comparar comportamentos.
- Identificar ausências.
- Identificar divergências.
- Identificar bugs.
- Classificar paridade.
- Registrar evidências.

**Entregável:** matriz completa de comparação.

## FASE 7 — Qualidade e testes

- Avaliar arquitetura.
- Avaliar segurança.
- Avaliar persistência.
- Avaliar concorrência.
- Avaliar protocolos.
- Criar estratégia de regressão.
- Identificar testes necessários.

**Entregável:** auditoria técnica e plano de testes.

## FASE 8 — Roadmap

- Classificar prioridades.
- Identificar dependências.
- Estimar complexidade.
- Organizar fases.
- Definir critérios de aceite.
- Separar funcionalidades oficiais de customizações.

**Entregável:** roadmap técnico consolidado.

## FASE 9 — Consolidação

- Revisar documentos.
- Eliminar duplicações.
- Verificar referências.
- Conferir consistência das matrizes.
- Atualizar índice.
- Registrar lacunas.
- Elaborar relatório executivo.

**Entregável:** base de conhecimento consolidada.

---

# 17. ESTRATÉGIA DE EXECUÇÃO POR AGENTE DE IA

O trabalho pode exigir múltiplos agentes especializados.

Caso o ambiente permita agentes paralelos, organize a investigação em especialidades independentes:

| Agente | Responsabilidade |
|---|---|
| Orquestrador | Coordenar o trabalho, consolidar descobertas e evitar duplicação. |
| Legacy Analyst | Analisar OpenWYD e branch `main`. |
| Rust Auditor | Analisar a implementação Rust. |
| Client Analyst | Investigar o cliente oficial em `client/`. |
| Protocol Engineer | Mapear pacotes, serialização e comunicação. |
| Reverse Engineer | Investigar binários, memória, funções e ferramentas. |
| Game Systems Analyst | Mapear mecânicas e regras do jogo. |
| Assets Specialist | Investigar sprites, meshes, texturas, animações e mapas. |
| Research Agent | Pesquisar referências externas e documentação. |
| QA Engineer | Planejar testes de paridade e regressão. |
| Documentation Agent | Organizar e consolidar a documentação. |

Cada agente deverá:

1. Receber um escopo definido.
2. Examinar as fontes disponíveis.
3. Registrar evidências.
4. Identificar incertezas.
5. Produzir documentação.
6. Evitar conclusões sem comprovação.
7. Comunicar dependências.
8. Entregar resultados ao orquestrador.

O orquestrador deverá reconciliar informações conflitantes e impedir que agentes diferentes documentem o mesmo comportamento de maneiras incompatíveis.

Não permita que agentes paralelos modifiquem simultaneamente os mesmos arquivos.

---

# 18. CRITÉRIOS DE CONCLUSÃO

A auditoria não será considerada concluída apenas porque os documentos foram criados.

Precisamos verificar:

- Todos os diretórios relevantes foram inventariados?
- Todas as funcionalidades identificadas no legado foram catalogadas?
- Todas as implementações Rust foram analisadas?
- Todas as funcionalidades conhecidas do cliente foram mapeadas?
- Os fluxos de comunicação foram investigados?
- Os protocolos foram documentados na medida permitida pelas evidências?
- As diferenças entre legado e Rust foram identificadas?
- Os problemas conhecidos foram investigados?
- As funcionalidades desconhecidas foram registradas?
- A documentação existente foi confrontada com o código?
- As referências externas foram classificadas?
- As possibilidades de engenharia reversa foram avaliadas?
- Os recursos gráficos foram inventariados?
- As possibilidades de edição e criação de itens foram documentadas?
- As dependências entre sistemas foram identificadas?
- O roadmap foi construído?
- Os documentos possuem referências rastreáveis?
- As incertezas estão claramente registradas?

Apresente uma porcentagem de cobertura da investigação somente quando existir uma metodologia explícita de medição.

Não declare 100% de cobertura sem comprovar o escopo analisado.

---

# 19. RESULTADO FINAL ESPERADO

Queremos alcançar uma compreensão técnica abrangente do WYD2.

Ao final, devemos saber:

**Sobre o legado:**
- Como cada sistema funciona.
- Onde está implementado.
- Quais regras são utilizadas.
- Quais comportamentos são esperados.
- Quais bugs ainda existem.
- Quais funcionalidades estão incompletas.

**Sobre Rust:**
- O que já foi portado.
- O que está funcionando.
- O que está incompleto.
- O que está incorreto.
- O que ainda não foi implementado.
- Quais mudanças são necessárias.
- Como atingir paridade funcional.

**Sobre o cliente:**
- Como funciona internamente.
- Quais funcionalidades possui.
- Como se comunica com o servidor.
- Quais protocolos utiliza.
- Quais mensagens envia e recebe.
- Quais eventos controla.
- Quais recursos podem ser modificados.
- Quais limitações existem.

**Sobre engenharia reversa:**
- Quais fontes estão disponíveis.
- Quais ferramentas podem ser utilizadas.
- Como analisar executáveis.
- Como analisar protocolos.
- Como investigar formatos proprietários.
- Como identificar funções e estruturas.
- Quais abordagens são mais eficientes.

**Sobre customização:**
- Como modificar interfaces.
- Como modificar gráficos.
- Como editar sprites.
- Como criar animações.
- Como editar meshes.
- Como criar modelos.
- Como modificar mapas.
- Como criar itens funcionais.
- Como criar armas.
- Como criar novos sets.
- Como adicionar conteúdo.

**Sobre o futuro:**
- Quais funcionalidades devem ser implementadas primeiro.
- Quais dependências precisam ser resolvidas.
- Quais problemas precisam ser corrigidos.
- Quais melhorias arquiteturais são necessárias.
- Quais customizações são viáveis.
- Como organizar a evolução do projeto.

---

# 20. PRIMEIRA AÇÃO — INICIAR A AUDITORIA

Comece imediatamente pela FASE 1.

Não faça perguntas genéricas antes de investigar o projeto.

Primeiro, descubra o que está disponível no ambiente e quais informações podem ser obtidas diretamente dos repositórios e arquivos existentes.

Execute a investigação progressivamente, documentando as descobertas desde o início.

Não espere terminar toda a auditoria para começar a produzir documentação.

Ao encontrar uma funcionalidade:

1. Registre sua existência.
2. Identifique suas fontes.
3. Mapeie seu funcionamento.
4. Identifique suas dependências.
5. Compare as implementações.
6. Registre diferenças.
7. Classifique o status.
8. Identifique lacunas.
9. Documente evidências.
10. Atualize a matriz.

**DIRETRIZ FINAL:**
Não queremos apenas portar o OpenWYD para Rust.
Queremos compreender profundamente o WYD2, reconstruir seu conhecimento técnico, reproduzir corretamente os comportamentos oficiais, identificar tudo o que ainda não foi implementado, corrigir os problemas históricos e criar uma base tecnológica que permita evoluir o servidor e, futuramente, o próprio cliente.
A prioridade é conhecimento completo, fidelidade comportamental, rastreabilidade técnica, qualidade de engenharia e capacidade de evolução.
Não deixe funcionalidades de fora simplesmente porque ainda não sabemos que elas existem. Descobri-las também faz parte da missão.