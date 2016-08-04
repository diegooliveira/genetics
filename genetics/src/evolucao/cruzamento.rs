

use std::vec::Vec;
use std::marker::PhantomData;

use populacao::*;
use evolucao::Evolucao;
use aleatorio::Aleatorio;
use evolucao::selecao::*;

/// Definição a forma de cruzamento de determinados Genes.
pub trait Cruzador<Gene> {
    /// Método que deverá cruzar os indivíduos *primeiro* e *segundo*.
    fn cruzar(&self, aleatorio: &mut Aleatorio, primeiro: &Gene, segundo: &Gene) -> (Gene, Gene);
}

pub struct CruzadorVetor;


/// Implementação do cruzador de vetores de elementos clonaveis.
impl<T> Cruzador<Vec<T>> for CruzadorVetor
    where T: Clone
{
    fn cruzar(&self,
              aleatorio: &mut Aleatorio,
              primeiro: &Vec<T>,
              segundo: &Vec<T>)
              -> (Vec<T>, Vec<T>) {

        debug!("Executando cruzamento");

        let mut f1 = Vec::new();
        let mut f2 = Vec::new();
        let qtd = primeiro.len();
        let mescla = (aleatorio.chance() * qtd as f64).round() as usize;

        for i in 0..qtd {
            if i < mescla {
                f1.push(primeiro[i].clone());
                f2.push(segundo[i].clone());
            } else {
                f2.push(primeiro[i].clone());
                f1.push(segundo[i].clone());
            }
        }

        (f1, f2)
    }
}

pub struct Cruzamento<Gene, Sel, Cruz>
    where Cruz: Cruzador<Gene>,
          Sel: Seletor<Gene>
{
    /// Modelo de seleção para cruzamento
    pub seletor: Sel,
    /// Implementação que faz o cruzamento propriamente dito
    pub cruzador: Cruz,
    /// Chance de cruzamento de 0 a 1
    pub chance_cruzamento: f64,

    /// Marcações fantasmas, serve apenas para forçar os tipos parametricos Gene e Apt
    pub phanton_gene: PhantomData<Gene>,
}

impl<Gene, Sel, Cruz> Cruzamento<Gene, Sel, Cruz>
    where Cruz: Cruzador<Gene>,
          Sel: Seletor<Gene>
{
    pub fn criar(seletor: Sel, cruzador: Cruz, chance_cruzamento: f64) -> Self {
        Cruzamento {
            seletor: seletor,
            cruzador: cruzador,
            chance_cruzamento: chance_cruzamento,

            phanton_gene: PhantomData,
        }
    }
}


impl<Gene, Sel, Cruz> Evolucao<Gene> for Cruzamento<Gene, Sel, Cruz>
    where Cruz: Cruzador<Gene>,
          Sel: Seletor<Gene>
{
    fn evoluir(&self, pop: &mut Populacao<Gene>) -> Populacao<Gene> {
        debug!("Evoluindo por cruzamento");
        let mut nova_geracao = pop.preparar_nova_geracao();
        let qtd = pop.individuos.len();
        let mut aleatorio = Aleatorio::criar();
        for idx in 0..qtd {

            if self.chance_cruzamento > aleatorio.chance() {
                trace!("Cruzando {}/{}", idx, qtd);
                let par = self.seletor.seleciona(pop);
                if par.is_some() {

                    let inds = par.unwrap();

                    let (f1, f2) = self.cruzador
                                       .cruzar(&mut aleatorio, &inds.0.genes, &inds.1.genes);

                    nova_geracao.adicionar(f1);
                    nova_geracao.adicionar(f2);


                    pop.reiterar(inds.0);
                    pop.reiterar(inds.1);

                }
            }

        }

        nova_geracao
    }
}

/// Macro auxiliar para cruzamento de arrays de um ponto
#[macro_export]
macro_rules! cruza_array_um_ponto {
    ($sorte: expr, $tipo:expr,$tamanho:expr,$primeiro: expr, $segundo:expr) => {
        {
            let mut a = [$tipo; $tamanho];
            let mut b = [$tipo; $tamanho];
            let posicao = ($sorte * $tamanho as f64) as usize;
            for i in 0..$tamanho {

                if i < posicao {
                    a[i] = $primeiro[i];
                    b[i] = $segundo[i];
                } else {
                    a[i] = $segundo[i];
                    b[i] = $primeiro[i];
                }
            }
            (a, b)
        }
    }
}

/// Macro auxiliar para cruzamento de arrays dois um ponto
#[macro_export]
macro_rules! cruza_array_dois_pontos {
    ($sorte_um: expr, $sorte_dois: expr, $tipo:expr,$tamanho:expr,$primeiro: expr, $segundo:expr) => {
        {

        	use std::mem;

            let mut a = [$tipo; $tamanho];
            let mut b = [$tipo; $tamanho];
            let mut posicao_um = ($sorte_um * $tamanho as f64) as usize;
            let mut posicao_dois = ($sorte_dois * $tamanho as f64) as usize;

			if posicao_dois < posicao_um {
				mem::swap(&mut posicao_dois, &mut posicao_um);
			}

            for i in 0..$tamanho {

                if i < posicao_um {
                    a[i] = $primeiro[i];
                    b[i] = $segundo[i];
                } else if i < posicao_dois {
                    a[i] = $segundo[i];
                    b[i] = $primeiro[i];
                } else {
                	a[i] = $primeiro[i];
                    b[i] = $segundo[i];
                }
            }

            (a, b)
        }
    }
}
