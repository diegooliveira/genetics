//! Arquivo principal do trabalho. Aqui acontece a ligação com os módulos necessários para
//! a execução do AG.

// Linking com a biblioteca externa de algoritmos genéticos que eu desenvolvi
#[macro_use]
extern crate genetics;


// Linking com uma biblioteca externa para processamento de parametros de linha de comando.
// http://kbknapp.github.io/clap-rs/clap/index.html
extern crate clap;

// Linkando com biblioteca de log(abstração)
#[macro_use]
extern crate log;

// Linkando com biblioteca de log(implementação)
extern crate simplelog;

// Linking com arquivos dentro desse projeto

/// Função Rastrigin no arquivo
mod rastrigin;
/// Funções Unimodais
mod unimodal;
/// Função Mutimodal
mod multimodal;

/// Processamento de parametros de linha de comando
mod configuracao;

// Trazendo o tipo Populacao para esse contexto
use genetics::populacao::Populacao;

// Trazendo o tipo que faz evolução mista para esse contexto. A evolução mista permite usar mais
// de uma estratégia de evolução, como por exemplo mutação/crossover/etc.
use genetics::evolucao::EvolucaoMista;

use genetics::evolucao::mutacao::Mutacao;
use genetics::evolucao::cruzamento::Cruzamento;
use genetics::evolucao::selecao::torneio::SelecaoPorTorneio;
use genetics::evolucao::selecao::roleta::SeletorPorRoleta;
use genetics::observador::ObservadorEvolucaoImprimeAptidao;
use genetics::objetivo::{ObjetivoMisto, NumeroMaximoIteracoes};
use genetics::genetico::Genetico;

macro_rules! resolve {
    ($cfg: expr, $tipo: expr) => {
        {

        	$tipo.processa(&$cfg);

            let mutagenico = $tipo;
            let aptidao = $tipo;
            let cruzador = $tipo;
            let criador = $tipo;

            let populacao = Populacao::criar(aptidao, &criador,
            		$cfg.tamanho_populacao, $tipo.operacao());

            let mut evolucao = EvolucaoMista::criar();
            if $cfg.chance_mutacao > 0.0 {
                evolucao.adicionar(Mutacao::criar($cfg.chance_mutacao, mutagenico));
            }

            if $cfg.chance_cruzamento > 0.0 {

                if "torneio" == &*$cfg.seletor {
                    evolucao.adicionar(Cruzamento::criar(SelecaoPorTorneio::criar(5),
                                                         cruzador,
                                                         $cfg.chance_cruzamento));
                } else {
                    evolucao.adicionar(Cruzamento::criar(SeletorPorRoleta::criar(),
                                                         cruzador,
                                                         $cfg.chance_cruzamento));
                }
            }

            let mut objetivo = ObjetivoMisto::criar();
            objetivo.adicionar(NumeroMaximoIteracoes::criar($cfg.geracoes));

            let mut genetico = Genetico::criar(populacao, evolucao, objetivo);
            genetico.observador(ObservadorEvolucaoImprimeAptidao);
            genetico.elitismo = $cfg.elitismo;
            let sol = genetico.buscar_solucao();
            if $cfg.print_solution {
            	println!("{}", $tipo.format(sol));
            }
        }
    }
}

// Função inicial do programa
fn main() {

    // Lendo os parametros de execução do algoritmo;
    if let Some(configuracao) = configuracao::ler() {

        // Ligando LOGs caso se queira acompanhar a execução do algoritmo
        ativa_log(configuracao.debug);
        match &*configuracao.funcao {
            "rastrigin_arranjo" => resolve!(configuracao, rastrigin::RastriginArranjo),
            "rastrigin_binario" => resolve!(configuracao, rastrigin::RastriginBinario),
            "unimodal_arranjo_um" => resolve!(configuracao, unimodal::UnimodalArranjoUm),
            "unimodal_arranjo_dois" => {
                resolve!(configuracao, unimodal::UnimodalArranjoDois::criar())
            }
            "multimodal_arranjo" => resolve!(configuracao, multimodal::MultimodalArranjo),
            "multimodal_arranjo_binario" => resolve!(configuracao, multimodal::MultimodalBinario),
            _ => panic!("Funcao não mapeada: {}", configuracao.funcao),
        }
    }
}

/// Função que ativa o LOG, útil para compreender o que o algoritmo está fazendo por dentro.
fn ativa_log(nivel: u64) {
    use simplelog::{TermLogger, CombinedLogger, LogLevelFilter};
    if nivel == 1 {
        CombinedLogger::init(vec![TermLogger::new(LogLevelFilter::Debug)]).unwrap();
    } else if nivel > 1 {
        CombinedLogger::init(vec![TermLogger::new(LogLevelFilter::Trace)]).unwrap();
    }
}
