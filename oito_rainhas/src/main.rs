
extern crate genetics;
extern crate rand;

use genetics::populacao::{
	Populacao,
	Operacao
};
use genetics::aptidao::Aptidao;
use genetics::evolucao::*;
use genetics::objetivo::*;
use genetics::evolucao::cruzamento::*;
use genetics::evolucao::mutacao::*;
use genetics::genetico::*;
use genetics::evolucao::selecao::torneio::SelecaoPorTorneio;
use genetics::aleatorio::*;


use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};

fn main() {

    // A população que será trabalhada
    let mut pop = Populacao::criar_vazia(AptidaoOitoRainhas, Operacao::Max);
    preencher_com_posicoes_aleatorias(&mut pop, 1000);

    // Preparando o modelo de evolução que estamos interessado
    let mut evolucao = EvolucaoMista::criar();
    evolucao.adicionar(Mutacao::criar(0.4, PosicoesRainhas));
    evolucao.adicionar(Cruzamento::criar(SelecaoPorTorneio::criar(5),
                                         PosicoesRainhas,
                                         0.9));


    // Configurando os objetivos do algoritmo
    let mut objetivo = ObjetivoMisto::criar();
    objetivo.adicionar(NumeroMaximoIteracoes::criar(1000));
    objetivo.adicionar(ObjetivoIntervalo::criar(-0.99, 0.01));

    // Executando o algoritmo genetico
    let genetico = Genetico::criar(pop, evolucao, objetivo);
    let solucao = genetico.buscar_solucao();
    println!("\n\tSolução {:?}\n", solucao);

    println!("\t+---+---+---+---+---+---+---+---+");
    for i in 0..8 {
        print!("\t| ");
        for j in 0..8 {

            if solucao[j] == i {
                print!("X");
            } else {
                print!(" ");
            }
            print!(" | ");
        }
        println!("\n\t+---+---+---+---+---+---+---+---+");
    }
}

struct PosicoesRainhas;

impl Mutagenico<[usize; 8]> for PosicoesRainhas {
    fn mutar(&self, genes: &[usize; 8], aleatorio: &mut Aleatorio) -> [usize; 8] {

        let linha = aleatorio.intervalo(0, 8);
        let coluna = aleatorio.intervalo(0, 8);

        let mut mutante = genes.clone();
        mutante[linha] = coluna;

        mutante
    }
}

impl Cruzador<[usize; 8]> for PosicoesRainhas {
    fn cruzar(&self,
              aleatorio: &mut Aleatorio,
              primeiro: &[usize; 8],
              segundo: &[usize; 8])
              -> ([usize; 8], [usize; 8]) {

        let mut a = [0usize; 8];
        let mut b = [0usize; 8];
        let posicao = (aleatorio.chance() * 8.0) as usize;
        for i in 0..8 {

            if i < posicao {
                a[i] = primeiro[i];
                b[i] = segundo[i];
            } else {
                a[i] = segundo[i];
                b[i] = primeiro[i];
            }
        }

        (a, b)
    }
}

pub struct MutagenicoOitoRainhas;

pub struct AptidaoOitoRainhas;

impl Aptidao<[usize; 8]> for AptidaoOitoRainhas {
    /// Calcula a quantidade de rainhas que se ameaçam
    fn calcular_aptidao(&self, genes: &[usize; 8]) -> f64 {


        let mut qtd_rainhas_ameacando = 0;


        for i in 0..8 {

            let posicao_rainha_atual = genes[i];

            for j in (i + 1)..8 {

                let posicao_rainha_adversaria = genes[j];

                if posicao_rainha_atual == posicao_rainha_adversaria {
                    qtd_rainhas_ameacando = qtd_rainhas_ameacando - 1;
                    continue;
                }

                let distancia = j - i;

                let ameaca_diagonal_subindo = posicao_rainha_atual + distancia;
                if ameaca_diagonal_subindo == posicao_rainha_adversaria {
                    qtd_rainhas_ameacando = qtd_rainhas_ameacando - 1;
                    continue;
                }

                if posicao_rainha_atual >= distancia {
                    let ameaca_diagonal_descendo = posicao_rainha_atual - distancia;
                    if ameaca_diagonal_descendo == posicao_rainha_adversaria {
                        qtd_rainhas_ameacando = qtd_rainhas_ameacando - 1;
                        continue;
                    }
                }
            }
        }
        qtd_rainhas_ameacando as f64
    }
}

fn preencher_com_posicoes_aleatorias(pop: &mut Populacao<[usize; 8]>, qtd: usize) {

    let intervalo_posicao = Range::new(0, 7);
    let mut randon = thread_rng();

    for _ in 0..qtd {
        let mut tabuleiro = [0; 8];
        for i in 0..8 {
            tabuleiro[i] = intervalo_posicao.ind_sample(&mut randon);
        }
        pop.adicionar(tabuleiro);
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use genetics::aptidao::Aptidao;

    #[test]
    fn deve_calcular_aptidao() {
        valida([1, 1, 1, 1, 1, 1, 1, 1], 28);
        valida([0, 0, 3, 1, 5, 2, 1, 6], 5);
        valida([1, 3, 5, 7, 2, 0, 6, 4], 0);
    }

    fn valida(posicoes: [usize; 8], qtd: usize) {
        let apt = AptidaoOitoRainhas;
        let ameacas = apt.calcular_aptidao(&posicoes);

        assert_eq!(-(qtd as isize), ameacas as isize);
    }
}