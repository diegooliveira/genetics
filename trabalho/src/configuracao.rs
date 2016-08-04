

use clap::{Arg, App, ArgMatches};
use std::process;

#[derive(Debug)]
pub struct Configuracao {
    pub funcao: String, // Nome da função que será maximizada
    pub chance_mutacao: f64, // Percentual de mutacação
    pub chance_cruzamento: f64, // Percentual de cruzamento
    pub modificador_cruzamento: String, // Modificador que pode ser aplicado ao cruzamento
    pub geracoes: usize, // Quantidade máxima de gerações para o algoritmo genético
    pub seletor: String, // Tipo de seleção a ser aplicada no cruzamento
    pub tamanho_populacao: usize, // Tamanho da população
    pub debug: u64, // Parametro que indica se deve ser logado informações de depuração
    pub print_solution: bool, // Parametro que informa se deve ser impresso o resultado encontrado
    pub elitismo: bool, // Ativa o elitismo
}

/// Processa os parametros de linha de comando.
pub fn ler() -> Option<Configuracao> {

    let parametros = prepara_parametros();
    if let Some(funcao) = parametros.value_of("funcao") {

        let mutacao = to_int(parametros.value_of("mutacao").unwrap_or("0"));
        let cruzamento = to_int(parametros.value_of("cruzamento").unwrap_or("0"));
        let geracoes = to_int(parametros.value_of("geracoes").unwrap_or("0"));

        return Some(Configuracao {
            funcao: funcao.into(),
            chance_mutacao: (mutacao as f64 / 100.0 as f64),
            chance_cruzamento: (cruzamento as f64 / 100.0 as f64),
            modificador_cruzamento: parametros.value_of("modificador-cruzamento")
                                              .unwrap()
                                              .into(),
            tamanho_populacao: to_int(parametros.value_of("populacao").unwrap_or("100")),
            geracoes: geracoes,
            seletor: parametros.value_of("seletor").unwrap_or("torneio").into(),
            debug: parametros.occurrences_of("debug"),
            print_solution: parametros.occurrences_of("imprime-solucao") > 0,
            elitismo: parametros.occurrences_of("elitismo") > 0,
        });
    }
    return None;
}

/// Configura todos os parametros aceitos pelo sistema
fn prepara_parametros<'a>() -> ArgMatches<'a> {
    App::new("Trabalho Inteligência Computacional")
        .author("Diego de Oliveira")
        .about("Does awesome things")
        .arg(Arg::with_name("debug")
                 .short("d")
                 .multiple(true)
                 .help("Ativa a impressão de informações de debug."))
        .arg(Arg::with_name("mutacao")
                 .long("mutacao")
                 .short("m")
                 .default_value("1")
                 .takes_value(true)
                 .help("Ativa a mutação"))
        .arg(Arg::with_name("cruzamento")
                 .long("cruzamento")
                 .short("c")
                 .default_value("9")
                 .takes_value(true)
                 .help("Ativa o cruzamento"))
        .arg(Arg::with_name("modificador-cruzamento")
                 .long("modificador-cruzamento")
                 .short("mc")
                 .possible_values(&["um-ponto", "dois-pontos"])
                 .default_value("um-ponto")
                 .takes_value(true)
                 .help("Aplica modificadores ao cruzamento"))
        .arg(Arg::with_name("geracoes")
                 .long("geracoes")
                 .short("g")
                 .default_value("100")
                 .takes_value(true)
                 .help("Quantidade máxima de gerações"))
        .arg(Arg::with_name("seletor")
                 .long("seletor")
                 .short("s")
                 .possible_values(&["torneio", "roleta"])
                 .takes_value(true)
                 .default_value("torneio")
                 .help("Tipo de seleção usada para o cruzamento"))
        .arg(Arg::with_name("populacao")
                 .long("populacao")
                 .short("p")
                 .default_value("100")
                 .takes_value(true)
                 .help("Tamanho da população inicial"))
        .arg(Arg::with_name("funcao")
                 .long("funcao")
                 .help("Função a ser processada")
                 .possible_values(&["rastrigin_arranjo",
                                    "rastrigin_binario",
                                    "unimodal_arranjo_um",
                                    "unimodal_arranjo_dois",
                                    "multimodal_arranjo",
                                    "multimodal_arranjo_binario"])
                 .short("funcao")
                 .takes_value(true)
                 .required(true))
        .arg(Arg::with_name("imprime-solucao")
                 .long("imprime-solucao")
                 .help("Nome do arquivo a ser salvo o log do processamento. Serão gerados dois \
                        arquivos, um com o sufxi '_parametros.txt' contendo os parametros da \
                        execução do algoritmo e outro com o sulfixo _fitness.csv.")
                 .takes_value(false))
        .arg(Arg::with_name("elitismo")
                 .long("elitismo")
                 .help("Ativa o elitismo.")
                 .short("e")
                 .takes_value(false))
        .arg(Arg::with_name("log")
                 .long("log")
                 .help("Nome do arquivo a ser salvo o log do processamento. Serão gerados dois \
                        arquivos, um com o sufxi '_parametros.txt' contendo os parametros da \
                        execução do algoritmo e outro com o sulfixo _fitness.csv.")
                 .short("l")
                 .takes_value(true))
        .get_matches()
}

fn to_int(valor: &str) -> usize {

    use std::str::FromStr;

    // Parseando o texto, caso de erro vamos tentar dar uma mensagem de erro amigavel
    let val = FromStr::from_str(valor);
    match val {
        Ok(val) => return val,
        Err(_) => {
            println!("Valor numerico inválido {}", valor);
            process::exit(-1);
        }
    }
}
