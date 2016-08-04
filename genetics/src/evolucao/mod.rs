//! As estruturas definidas aqui permitem montar um algoritmos genéticos elaborados;
//!

/// Modelos de cruzamento para uso
pub mod cruzamento;
/// Modelos de mutações disponíveis
pub mod mutacao;
/// Métodos de seleção para uso no cruzamento
pub mod selecao;

use populacao::*;



pub trait Evolucao<Gene> {
    fn evoluir(&self, pop: &mut Populacao<Gene>) -> Populacao<Gene>;
}

pub struct EvolucaoMista<Gene> {
    evolucoes: Vec<Box<Evolucao<Gene>>>,
}

impl<Gene> EvolucaoMista<Gene> {
    pub fn criar() -> Self {
        EvolucaoMista { evolucoes: Vec::new() }
    }

    pub fn adicionar<Evo>(&mut self, evolucao: Evo)
        where Evo: Evolucao<Gene> + 'static
    {
        self.evolucoes.push(Box::new(evolucao));
    }
}

impl<Gene> Evolucao<Gene> for EvolucaoMista<Gene> {
    fn evoluir(&self, populacao: &mut Populacao<Gene>) -> Populacao<Gene> {

        let mut nova_populacao = populacao.preparar_nova_geracao();

        for ev in self.evolucoes.iter() {

            let pop = ev.evoluir(populacao);
            nova_populacao.reter(pop);
        }

        nova_populacao
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use populacao::*;
    use aptidao::*;

    #[test]
    fn deve_executar_todas_evolucoes() {

        let mut ev = EvolucaoMista::criar();
        ev.adicionar(EvolucaoTest { valor: 2 });
        ev.adicionar(EvolucaoTest { valor: 3 });
        ev.adicionar(EvolucaoTest { valor: 4 });


        let mut pop = Populacao::criar_vazia(AptidaoSimples, Operacao::Max);
        pop.adicionar(1);


        let new_pop = ev.evoluir(&mut pop);
        assert_eq!(3, new_pop.individuos.len());

        assert_eq!(2, new_pop.individuos[0].genes);
        assert_eq!(3, new_pop.individuos[1].genes);
        assert_eq!(4, new_pop.individuos[2].genes);

    }

    struct EvolucaoTest {
        valor: usize,
    }

    impl Evolucao<usize> for EvolucaoTest {
        fn evoluir(&self, pop: &mut Populacao<usize>) -> Populacao<usize> {


            let mut nova_populacao = pop.preparar_nova_geracao();

            for _ in pop.individuos.iter() {

                nova_populacao.adicionar(self.valor);
            }

            nova_populacao
        }
    }
}
