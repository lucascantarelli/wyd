# Análise de dados completa do WYD

Atualmente temos dentro da pasta client/ os arquivos do wyd

dentro do banco de dados temos os dados desses arquivos cadastrados, com migrations populando dados, tudo seguindo o fluxo correto.

eu tenho uma hipótese sobre como o servidor legado do wyd tratava o banco de dados dos itens.

o cliente lia arquivos como itemlist,skillist, arquivos .bin, etc, e eles serviam como o banco de dados de items, skills, etc.
para atualizar esses arquivos, o servidor legado do wyd tinha um sistema de atualização que baixava os arquivos atualizados e os substituía no cliente.
existiam programas especificos para editar esses arquivos, e era gerado o pacote e o launcher atualizada.

sendo que, os servidores legados não usavam banco de dados, eles usavam arquivos binários como banco de dados, e o cliente lia esses arquivos para obter os dados.

mas, atualmente, estamos mudando a forma de como esses itens são tratados, e estamos usando um banco de dados para armazenar os dados dos itens, skills, etc.
atualmente existem muitos ruidos e redundancias dentro dos dados desses binários, minha idéia é entendermos o real funcionamento do servidor legado, e como ele tratava os dados, para que possamos melhorar a forma como estamos tratando esses dados atualmente.

eu sei que nos itens existem campos onde informam certas funções especificas, como por exemplo, o campo "type" que informa se o item é uma arma, armadura, consumível, etc.
os npcs, se não me engano tb são configurados em arquivos binários, como itens, existem várias nuancias e dados expecificos, como meshs usadas, formato do npc, modelo de item, etc.

minha idéia é mapear o funcionamento do servidor legado, entender como ele tratava os dados, e como podemos melhorar a forma como estamos tratando esses dados, melhorar a arquitetua do banco de dados.
o cliente e os arquivos bin, infelizmente são uma estrutura fechada, não conseguiremos mudar a forma que cliente le os arquivos, então minha idéia é:

- sabendo que bin é limitado, tem um tamanho fechado e sabemos esse tamanho, é criar a tabela de itens identica ao bin, exatamente limitada, como ele, queria separar os campos e criar tabelas relacionadas, para poder realmente incluir descrições do que é cada coisa, saber os espaços vazios dos bins, e como podemos melhorar a forma como estamos tratando esses dados atualmente.
- minha idéia e que nossa tabela seja exatamente igual ao bin, mas com tabelas relacionadas, para que possamos ter uma visão mais clara do que cada campo significa, e como podemos melhorar a forma como estamos tratando esses dados atualmente.
- por exempli, efeitos dos itens, no lugar de ter apenas um dado dentro do campo de efeito, termos uma tabela com todos os efeitos, e uma tabela relacionada com os itens, para que possamos ter uma visão mais clara do que cada efeito significa, e como podemos melhorar a forma como estamos tratando esses dados atualmente.
- quero que vc proponha uma real remodelagem do banco de dados, com tabelas relacionadas, para que possamos ter uma visão mais clara do que cada campo significa, e como podemos melhorar a forma como estamos tratando esses dados atualmente.
- isso deve ser feito com todos os dados do jogo, como itens, skills, npcs, etc.
- minha idéia é que, quando queira, por exemplo, criar um item novo, eu apenas cadastre no banco de dados, o sistema do launcher ou a geração de pacote seja capaz de ler o banco de dados e gerar corretamente todos os arquivos binários(isso é apenas exemplo de como eu vou usar, não deve ser tarefa aqui nesse documento, apenas quero que vc entenda a ideia de como vamos usar o banco de dados, e como podemos melhorar a forma como estamos tratando esses dados atualmente).
- para isso ser realizado, devemos entender como o servidor legado tratava os dados, e como podemos melhorar a forma como estamos tratando esses dados atualmente.
- deixar nosso banco de daods mais organizado, com tabelas relacionadas, para que possamos ter uma visão mais clara do que cada campo significa, e como podemos melhorar a forma como estamos tratando esses dados atualmente.
- esses dados devem ser identicos ao bin, mas com tabelas relacionadas, para que possamos ter uma visão mais clara do que cada campo significa, e como podemos melhorar a forma como estamos tratando esses dados atualmente.
- criar neste documento seções de acompanhamento, informações confirmadas, informações a confirmar, e informações a pesquisar, para que possamos ter uma visão mais clara do que cada campo significa, e como podemos melhorar a forma como estamos tratando esses dados atualmente.
- Manter essa documentação viva, escerver apenas nela, não criar outros documentos, para que possamos ter uma visão mais clara do que cada campo significa, e como podemos melhorar a forma como estamos tratando esses dados atualmente.

## Documentação

Já temos diversas documentações de como tudo funciona, precisamos realmente confirmar tudo e estruturar no banco de dados

- docs/
- tools/
- roadpmap-rust/