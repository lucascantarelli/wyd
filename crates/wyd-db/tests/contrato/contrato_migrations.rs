//! Alvo `contrato_migrations` — a **forma** da série, lida como texto, sem banco.
//!
//! Página dona: `planejamento-1.0.0/wyd-db/tests/alvos/contrato-migrations.md`.
//! Regras da camada: `planejamento-1.0.0/wyd-db/tests/specs/contrato.md` (não abre conexão, usa o
//! mesmo parser do runner, um defeito por fixture, mensagem nomeia arquivo, linha e invariante).
//!
//! Sete invariantes, um por capacidade:
//!
//! | `CAP` | Invariante | Defeito que ele pega |
//! |---|---|---|
//! | `CAP-01` | série contígua a partir de `0001` | furo ou repetição de número |
//! | `CAP-02` | o `down` derruba toda tabela que o `up` criou | tabela sobrevivendo ao `down` |
//! | `CAP-03` | nenhuma FK aponta para arquivo posterior | `REFERENCES` para tabela do futuro |
//! | `CAP-04` | seed só depois do `CREATE TABLE` da tabela | `INSERT` antes da tabela |
//! | `CAP-05` | seed só em arquivo `_seed_` | `INSERT` em arquivo de DDL |
//! | `CAP-06` | toda constraint tem nome | `PRIMARY KEY`/`CHECK`/`UNIQUE` anônima |
//! | `CAP-07` | nenhum tipo da lista proibida | `varchar`, `serial`, `timestamp` sem fuso |
//!
//! Cada invariante é provado **duas vezes**: na fixture que passa (`resources/serie/valida/`) e na
//! fixture que viola só aquele invariante (`resources/serie/invalida/<defeito>/`). Fixture que viola
//! dois invariantes prova que o alvo reprova, não qual regra ele confere.
//!
//! **Limite medido em 2026-11-27, declarado de propósito:** contra a série viva (`migrations/`, 28
//! arquivos), os invariantes `CAP-02`–`CAP-07` passam; `CAP-01` **não** — faltam `0016`, `0017`,
//! `0018`, `0020`, `0023` e `0027`. O alvo confere a regra de contiguidade nas fixtures e fixa o
//! estado medido da série viva num teste próprio, para a lacuna aparecer em vez de passar calada.
//! Enquanto os seis arquivos faltarem, `CAP-01` não é capacidade coberta.
//!
//! O nome e os marcadores de cada arquivo são validados pelo **parser do runner**
//! (`wyd_db::migrate::migration::file::parse`), não por um segundo parser; os analisadores daqui só
//! localizam o texto de cada invariante.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use wyd_db::migrate::migration::file;

/// Série viva do crate.
const SERIE_VIVA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
/// Fixtures do contrato.
const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/resources/serie");

/// Um arquivo da série já validado pelo parser do runner.
#[derive(Debug)]
struct Arquivo {
    versao: u32,
    nome: String,
    up: String,
    down: String,
    /// Linhas antes do `-- migrate:up`, para o número da linha ser o do arquivo.
    deslocamento: usize,
}

impl Arquivo {
    /// Linha absoluta de um deslocamento em bytes dentro do `up`.
    fn linha_de(&self, deslocamento: usize) -> usize {
        self.deslocamento + self.up[..deslocamento].matches('\n').count() + 1
    }

    /// Linhas do `up` com o comentário removido e o número da linha no arquivo.
    fn linhas_de_codigo(&self) -> Vec<(usize, String)> {
        sem_comentarios(&self.up)
            .split('\n')
            .enumerate()
            .map(|(indice, linha)| (self.deslocamento + indice + 1, linha.to_owned()))
            .collect()
    }
}

/// Um invariante violado, com arquivo, linha e regra.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Defeito {
    Furo {
        arquivo: String,
        ausente: u32,
    },
    Repetida {
        arquivo: String,
        versao: u32,
    },
    DownIncompleto {
        arquivo: String,
        linha: usize,
        tabela: String,
    },
    FkFutura {
        arquivo: String,
        linha: usize,
        destino: String,
        criada_em: u32,
    },
    FkSemTabela {
        arquivo: String,
        linha: usize,
        destino: String,
    },
    SeedAntes {
        arquivo: String,
        linha: usize,
        tabela: String,
    },
    SeedSemMarca {
        arquivo: String,
        linha: usize,
        tabela: String,
    },
    ConstraintAnonima {
        arquivo: String,
        linha: usize,
    },
    TipoProibido {
        arquivo: String,
        linha: usize,
        tipo: &'static str,
    },
}

impl fmt::Display for Defeito {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Furo { arquivo, ausente } => write!(
                f,
                "{arquivo}: série não contígua: falta o arquivo {ausente:04}"
            ),
            Self::Repetida { arquivo, versao } => write!(
                f,
                "{arquivo}: série não contígua: {versao:04} aparece mais de uma vez"
            ),
            Self::DownIncompleto {
                arquivo,
                linha,
                tabela,
            } => write!(
                f,
                "{arquivo}:{linha}: tabela sobrevive ao down: {tabela} nasce no up e não é derrubada"
            ),
            Self::FkFutura {
                arquivo,
                linha,
                destino,
                criada_em,
            } => write!(
                f,
                "{arquivo}:{linha}: FK para arquivo posterior: {destino} nasce em {criada_em:04}"
            ),
            Self::FkSemTabela {
                arquivo,
                linha,
                destino,
            } => write!(
                f,
                "{arquivo}:{linha}: FK para tabela que a série não cria: {destino}"
            ),
            Self::SeedAntes {
                arquivo,
                linha,
                tabela,
            } => write!(
                f,
                "{arquivo}:{linha}: seed antes da tabela: INSERT em {tabela} antes do CREATE TABLE"
            ),
            Self::SeedSemMarca {
                arquivo,
                linha,
                tabela,
            } => write!(
                f,
                "{arquivo}:{linha}: seed em arquivo de DDL: INSERT em {tabela} fora de arquivo _seed_"
            ),
            Self::ConstraintAnonima { arquivo, linha } => write!(
                f,
                "{arquivo}:{linha}: constraint anônima: falta CONSTRAINT nome antes da declaração"
            ),
            Self::TipoProibido {
                arquivo,
                linha,
                tipo,
            } => write!(f, "{arquivo}:{linha}: tipo proibido: {tipo}"),
        }
    }
}

/// Texto do `up` sem comentários, preservando comprimento e número de linhas.
///
/// Prosa em comentário cita `references` e nomes de coluna que parecem tipo; sem esta limpeza, o
/// alvo acusaria defeito onde não há SQL.
fn sem_comentarios(texto: &str) -> String {
    let mut saida = String::with_capacity(texto.len() + 1);
    for linha in texto.split('\n') {
        let inicio = linha.trim_start();
        let corte = if inicio.starts_with("--") {
            Some(0)
        } else {
            linha.find("--").filter(|posicao| {
                *posicao == 0 || linha.as_bytes()[posicao - 1].is_ascii_whitespace()
            })
        };
        match corte {
            Some(posicao) => {
                saida.push_str(&linha[..posicao]);
                saida.push_str(&" ".repeat(linha.len() - posicao));
            }
            None => saida.push_str(linha),
        }
        saida.push('\n');
    }
    saida
}

/// Primeira ocorrência de `alvo` como palavra inteira, em minúsculas.
fn palavra(texto: &str, alvo: &str) -> Option<usize> {
    let baixo = texto.to_ascii_lowercase();
    let mut inicio = 0;
    while let Some(posicao) = baixo[inicio..].find(alvo) {
        let absoluto = inicio + posicao;
        let antes_ok = absoluto == 0 || !baixo.as_bytes()[absoluto - 1].is_ascii_alphanumeric();
        let fim = absoluto + alvo.len();
        let depois_ok = fim >= baixo.len() || !baixo.as_bytes()[fim].is_ascii_alphanumeric();
        if antes_ok && depois_ok {
            return Some(absoluto);
        }
        inicio = absoluto + alvo.len();
    }
    None
}

/// Nome qualificado que segue `chave` (por exemplo, `create table`), pulando `if not exists`.
fn nome_apos(texto: &str, chave: &str) -> Vec<(usize, String)> {
    let baixo = texto.to_ascii_lowercase();
    let mut achados = Vec::new();
    let mut inicio = 0;
    while let Some(posicao) = baixo[inicio..].find(chave) {
        let mut cursor = inicio + posicao + chave.len();
        while cursor < baixo.len() && baixo.as_bytes()[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if baixo[cursor..].starts_with("if not exists") {
            cursor += "if not exists".len();
            while cursor < baixo.len() && baixo.as_bytes()[cursor].is_ascii_whitespace() {
                cursor += 1;
            }
        }
        let nome: String = texto[cursor..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '.')
            .collect();
        if !nome.is_empty() {
            achados.push((cursor, nome));
        }
        inicio = cursor.max(inicio + posicao + chave.len());
    }
    achados
}

/// Tabelas derrubadas no `down`, expandindo a lista de um mesmo `DROP TABLE a, b;`.
fn tabelas_derrubadas(down: &str) -> Vec<String> {
    let baixo = down.to_ascii_lowercase();
    let mut nomes = Vec::new();
    let mut inicio = 0;
    while let Some(posicao) = baixo[inicio..].find("drop table") {
        let mut cursor = inicio + posicao + "drop table".len();
        while cursor < baixo.len() && baixo.as_bytes()[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if baixo[cursor..].starts_with("if exists") {
            cursor += "if exists".len();
        }
        let fim = baixo[cursor..]
            .find(';')
            .map_or(baixo.len(), |p| cursor + p);
        for parte in down[cursor..fim].split(',') {
            let nome = parte.trim();
            if !nome.is_empty() {
                nomes.push(nome.to_owned());
            }
        }
        inicio = fim;
    }
    nomes
}

/// Lê a série de um diretório, validando nome e marcadores pelo parser do runner.
fn carregar(dir: &Path) -> Vec<Arquivo> {
    let mut caminhos: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|erro| panic!("diretório da série ilegível {}: {erro}", dir.display()))
        .filter_map(Result::ok)
        .map(|entrada| entrada.path())
        .filter(|caminho| caminho.extension().is_some_and(|ext| ext == "sql"))
        .collect();
    caminhos.sort();

    assert!(
        !caminhos.is_empty(),
        "série vazia: nenhum `.sql` em {}",
        dir.display()
    );

    caminhos
        .into_iter()
        .map(|caminho| {
            let conteudo = fs::read_to_string(&caminho)
                .unwrap_or_else(|erro| panic!("arquivo ilegível {}: {erro}", caminho.display()));
            let stem = caminho
                .file_stem()
                .and_then(|nome| nome.to_str())
                .unwrap_or_else(|| panic!("nome de arquivo não UTF-8: {}", caminho.display()));
            let bruto =
                file::parse(stem, &conteudo).unwrap_or_else(|erro| panic!("{stem}.sql: {erro}"));
            let deslocamento = conteudo
                .lines()
                .position(|linha| linha == "-- migrate:up")
                .map_or(0, |indice| indice + 1);
            Arquivo {
                versao: bruto.version.get(),
                nome: format!("{stem}.sql"),
                up: bruto.up.as_ref().to_owned(),
                down: bruto.down.as_ref().to_owned(),
                deslocamento,
            }
        })
        .collect()
}

/// `CAP-01` — versões `1..=N`, sem furo nem repetição.
fn contiguidade(serie: &[Arquivo]) -> Vec<Defeito> {
    let mut defeitos = Vec::new();
    let ultima = serie.last().map_or(0, |arquivo| arquivo.versao);
    for ausente in 1..=ultima {
        if serie.iter().any(|arquivo| arquivo.versao == ausente) {
            continue;
        }
        let seguinte = serie
            .iter()
            .find(|arquivo| arquivo.versao > ausente)
            .map_or("fim da série", |arquivo| arquivo.nome.as_str());
        defeitos.push(Defeito::Furo {
            arquivo: seguinte.to_owned(),
            ausente,
        });
    }
    for (indice, arquivo) in serie.iter().enumerate() {
        if serie[..indice]
            .iter()
            .any(|outro| outro.versao == arquivo.versao)
        {
            defeitos.push(Defeito::Repetida {
                arquivo: arquivo.nome.clone(),
                versao: arquivo.versao,
            });
        }
    }
    defeitos
}

/// `CAP-02` — toda tabela criada no `up` é derrubada no `down` do mesmo arquivo.
fn down_completo(serie: &[Arquivo]) -> Vec<Defeito> {
    let mut defeitos = Vec::new();
    for arquivo in serie {
        let derrubadas = tabelas_derrubadas(&sem_comentarios(&arquivo.down));
        for (posicao, tabela) in nome_apos(&sem_comentarios(&arquivo.up), "create table") {
            if !derrubadas.contains(&tabela) {
                defeitos.push(Defeito::DownIncompleto {
                    arquivo: arquivo.nome.clone(),
                    linha: arquivo.linha_de(posicao),
                    tabela,
                });
            }
        }
    }
    defeitos
}

/// `CAP-03` — `REFERENCES` só para tabela já criada em arquivo igual ou anterior.
fn fk_sem_futuro(serie: &[Arquivo]) -> Vec<Defeito> {
    let mut criadas: Vec<(String, u32)> = Vec::new();
    for arquivo in serie {
        for (_, tabela) in nome_apos(&sem_comentarios(&arquivo.up), "create table") {
            criadas.push((tabela, arquivo.versao));
        }
    }

    let mut defeitos = Vec::new();
    for arquivo in serie {
        for (posicao, destino) in nome_apos(&sem_comentarios(&arquivo.up), "references") {
            match criadas.iter().find(|(tabela, _)| *tabela == destino) {
                Some((_, versao)) if *versao > arquivo.versao => defeitos.push(Defeito::FkFutura {
                    arquivo: arquivo.nome.clone(),
                    linha: arquivo.linha_de(posicao),
                    destino,
                    criada_em: *versao,
                }),
                Some(_) => {}
                None => defeitos.push(Defeito::FkSemTabela {
                    arquivo: arquivo.nome.clone(),
                    linha: arquivo.linha_de(posicao),
                    destino,
                }),
            }
        }
    }
    defeitos
}

/// `CAP-04` — `INSERT` só depois do `CREATE TABLE` da tabela que ele preenche.
fn seed_depois_da_tabela(serie: &[Arquivo]) -> Vec<Defeito> {
    let mut defeitos = Vec::new();
    for arquivo in serie {
        for (posicao, tabela) in nome_apos(&sem_comentarios(&arquivo.up), "insert into") {
            let criada_antes_no_arquivo =
                nome_apos(&sem_comentarios(&arquivo.up)[..posicao], "create table")
                    .into_iter()
                    .any(|(_, nome)| nome == tabela);
            let criada_antes_na_serie = serie
                .iter()
                .filter(|outro| outro.versao < arquivo.versao)
                .flat_map(|outro| nome_apos(&sem_comentarios(&outro.up), "create table"))
                .any(|(_, nome)| nome == tabela);
            if !criada_antes_no_arquivo && !criada_antes_na_serie {
                defeitos.push(Defeito::SeedAntes {
                    arquivo: arquivo.nome.clone(),
                    linha: arquivo.linha_de(posicao),
                    tabela,
                });
            }
        }
    }
    defeitos
}

/// `CAP-05` — `INSERT` só em arquivo cujo nome tem `_seed_`.
fn seed_em_arquivo_proprio(serie: &[Arquivo]) -> Vec<Defeito> {
    let mut defeitos = Vec::new();
    for arquivo in serie {
        if arquivo.nome.contains("_seed_") {
            continue;
        }
        for (posicao, tabela) in nome_apos(&sem_comentarios(&arquivo.up), "insert into") {
            defeitos.push(Defeito::SeedSemMarca {
                arquivo: arquivo.nome.clone(),
                linha: arquivo.linha_de(posicao),
                tabela,
            });
        }
    }
    defeitos
}

/// `CAP-06` — toda constraint declara `CONSTRAINT nome` no mesmo item.
fn constraint_nomeada(serie: &[Arquivo]) -> Vec<Defeito> {
    let palavras = [
        "primary key",
        "unique",
        "check",
        "foreign key",
        "references",
    ];
    let mut defeitos = Vec::new();
    for arquivo in serie {
        let mut dentro = false;
        let mut nomeada = false;
        for (linha, texto) in arquivo.linhas_de_codigo() {
            let limpo = texto.trim();
            if !dentro {
                if palavra(limpo, "create table").is_some() {
                    dentro = true;
                    nomeada = false;
                }
                continue;
            }
            if limpo.starts_with(");") {
                dentro = false;
                continue;
            }
            if limpo.is_empty() {
                continue;
            }
            if palavra(limpo, "constraint").is_some() {
                nomeada = true;
            }
            if !nomeada && palavras.iter().any(|p| palavra(limpo, p).is_some()) {
                defeitos.push(Defeito::ConstraintAnonima {
                    arquivo: arquivo.nome.clone(),
                    linha,
                });
            }
            if limpo.ends_with(',') {
                nomeada = false;
            }
        }
    }
    defeitos
}

/// `CAP-07` — nenhum tipo da lista proibida em coluna.
fn tipo_permitido(serie: &[Arquivo]) -> Vec<Defeito> {
    let mut defeitos = Vec::new();
    for arquivo in serie {
        let mut dentro = false;
        let mut comeca_item = false;
        for (linha, texto) in arquivo.linhas_de_codigo() {
            let limpo = texto.trim();
            let baixo = limpo.to_ascii_lowercase();
            if !dentro {
                if baixo.starts_with("create table") {
                    dentro = true;
                    comeca_item = true;
                }
                continue;
            }
            if baixo.starts_with(");") {
                dentro = false;
                continue;
            }
            if limpo.is_empty() {
                continue;
            }
            let primeiro_da_coluna = comeca_item;
            comeca_item = limpo.ends_with(',');
            if !primeiro_da_coluna {
                continue;
            }
            if baixo.starts_with("constraint")
                || baixo.starts_with("primary key")
                || baixo.starts_with("unique")
                || baixo.starts_with("check")
                || baixo.starts_with("foreign key")
                || baixo.starts_with("references")
            {
                continue;
            }
            // Só o que vem depois do nome da coluna é tipo: `affect_time` não é `time`.
            let tipo = match limpo.split_once(char::is_whitespace) {
                Some((_, resto)) => resto.trim_start(),
                None => continue,
            };
            for (palavra_proibida, rotulo) in [
                ("serial", "serial"),
                ("bigserial", "serial"),
                ("smallserial", "serial"),
                ("varchar", "varchar"),
                ("money", "money"),
                ("enum", "enum"),
                ("time", "time"),
            ] {
                if palavra(tipo, palavra_proibida).is_some() {
                    defeitos.push(Defeito::TipoProibido {
                        arquivo: arquivo.nome.clone(),
                        linha,
                        tipo: rotulo,
                    });
                }
            }
            if let Some(posicao) = palavra(tipo, "timestamp") {
                let depois = tipo[posicao + "timestamp".len()..].trim_start();
                let com_fuso = depois.starts_with("tz")
                    || depois.starts_with("with time zone")
                    || depois.starts_with("with timezone");
                if !com_fuso {
                    defeitos.push(Defeito::TipoProibido {
                        arquivo: arquivo.nome.clone(),
                        linha,
                        tipo: "timestamp sem fuso",
                    });
                }
            }
        }
    }
    defeitos
}

/// Todos os invariantes de uma vez, na ordem dos `CAP`.
fn avaliar(serie: &[Arquivo]) -> Vec<Defeito> {
    let mut defeitos = contiguidade(serie);
    defeitos.extend(down_completo(serie));
    defeitos.extend(fk_sem_futuro(serie));
    defeitos.extend(seed_depois_da_tabela(serie));
    defeitos.extend(seed_em_arquivo_proprio(serie));
    defeitos.extend(constraint_nomeada(serie));
    defeitos.extend(tipo_permitido(serie));
    defeitos
}

fn fixture(nome: &str) -> Vec<Arquivo> {
    carregar(&Path::new(FIXTURES).join(nome))
}

fn conferir(nome: &str) -> Vec<String> {
    avaliar(&fixture(nome))
        .into_iter()
        .map(|defeito| defeito.to_string())
        .collect()
}

#[test]
fn serie_valida_passa_nos_sete_invariantes() {
    let problemas = conferir("valida");
    assert!(
        problemas.is_empty(),
        "a fixture boa violou invariante: {problemas:#?}"
    );
}

#[test]
fn cap01_furo_reprova() {
    let problemas = conferir("invalida/furo");
    assert_eq!(problemas.len(), 1, "{problemas:#?}");
    assert!(
        problemas[0].contains("série não contígua"),
        "{problemas:#?}"
    );
    assert!(problemas[0].contains("0002"), "{problemas:#?}");
}

#[test]
fn cap02_down_incompleto_reprova() {
    let problemas = conferir("invalida/down-incompleto");
    assert_eq!(problemas.len(), 1, "{problemas:#?}");
    assert!(
        problemas[0].contains("tabela sobrevive ao down: fixture.extra"),
        "{problemas:#?}"
    );
    assert!(
        problemas[0].contains("0001_fixture_base.sql:"),
        "{problemas:#?}"
    );
}

#[test]
fn cap03_fk_futura_reprova() {
    let problemas = conferir("invalida/fk-futura");
    assert_eq!(problemas.len(), 1, "{problemas:#?}");
    assert!(
        problemas[0].contains("FK para arquivo posterior"),
        "{problemas:#?}"
    );
    assert!(problemas[0].contains("fixture.b"), "{problemas:#?}");
}

#[test]
fn cap04_seed_antes_da_tabela_reprova() {
    let problemas = conferir("invalida/seed-antes");
    assert_eq!(problemas.len(), 1, "{problemas:#?}");
    assert!(
        problemas[0].contains("seed antes da tabela"),
        "{problemas:#?}"
    );
}

#[test]
fn cap05_seed_em_arquivo_de_ddl_reprova() {
    let problemas = conferir("invalida/seed-sem-marca");
    assert_eq!(problemas.len(), 1, "{problemas:#?}");
    assert!(
        problemas[0].contains("seed em arquivo de DDL"),
        "{problemas:#?}"
    );
}

#[test]
fn cap06_constraint_anonima_reprova() {
    let problemas = conferir("invalida/constraint-anonima");
    assert_eq!(problemas.len(), 1, "{problemas:#?}");
    assert!(
        problemas[0].contains("constraint anônima"),
        "{problemas:#?}"
    );
}

#[test]
fn cap07_tipo_proibido_reprova() {
    let problemas = conferir("invalida/tipo-proibido");
    assert_eq!(problemas.len(), 1, "{problemas:#?}");
    assert!(
        problemas[0].contains("tipo proibido: varchar"),
        "{problemas:#?}"
    );
}

/// Estado medido da série viva: forma completa, contiguidade pendente.
///
/// Este teste **não** declara `CAP-01` coberto: ele fixa o que existe hoje para a lacuna dos seis
/// arquivos aparecer no dia em que a série mudar, em vez de envelhecer calada.
#[test]
fn serie_viva_respeita_a_forma_com_contiguidade_pendente() {
    let serie = carregar(Path::new(SERIE_VIVA));

    let forma: Vec<String> = [
        down_completo(&serie),
        fk_sem_futuro(&serie),
        seed_depois_da_tabela(&serie),
        seed_em_arquivo_proprio(&serie),
        constraint_nomeada(&serie),
        tipo_permitido(&serie),
    ]
    .concat()
    .into_iter()
    .map(|defeito| defeito.to_string())
    .collect();
    assert!(forma.is_empty(), "série viva viola invariante: {forma:#?}");

    let ultima = serie.last().map_or(0, |arquivo| arquivo.versao);
    let faltando: Vec<u32> = (1..=ultima)
        .filter(|versao| !serie.iter().any(|arquivo| arquivo.versao == *versao))
        .collect();
    assert_eq!(
        faltando,
        vec![16, 17, 18, 20, 23, 27],
        "a série viva mudou: atualize a lacuna de `CAP-01` na página do alvo"
    );
    assert_eq!(
        contiguidade(&serie).len(),
        faltando.len(),
        "o número de furos medidos diverge da lacuna declarada"
    );
}
