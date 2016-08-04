
use evolucao::*;
use populacao::*;
use objetivo::*;
use observador::*;


/// Implementação do algoritmo genetico simplificado. Ela está dessa forma para
/// que seja possível alterar pedaços sem a necessidade de ficar alterando essa
/// implementação, apenas trocando uma das partes que se deseja.
pub struct Genetico<Gene, Evol, Obj>
    where Evol: Evolucao<Gene>, // Uma implementação que consiga evoluir o tipo T
          Obj: Objetivo<Gene> // Uma função objetiva que determina quando paramos as evoluções
{
    /// A população que se deseja evoluir
    pub populacao: Populacao<Gene>,
    /// O processo de evolução aplicado a população
    pub evolucao: Evol,
    /// O objetivo que se deseja atingir
    pub objetivo: Obj,
    /// O observador das evoluções
    pub observador: Box<ObservadorEvolucao<Gene>>,
    /// Informação se deve ou não ser utilizado elitismo entre as gerações.
    pub elitismo: bool,
}

/// Deve ler: Uma implementação que serve pera qualquer tipo de Gene/Aptidao/Objetivo/Evolução
impl<Gene, Evol, Obj> Genetico<Gene, Evol, Obj>
    where Evol: Evolucao<Gene>,
          Obj: Objetivo<Gene>
{
    /// Cria uma nova instância do algoritmo genético
    pub fn criar(pop: Populacao<Gene>, evolucao: Evol, objetivo: Obj) -> Self {
        Genetico {
            populacao: pop,
            evolucao: evolucao,
            objetivo: objetivo,
            observador: Box::new(ObservadorEvolucaoVazio),
            elitismo: false,
        }
    }

    /// Define um Observador para a evolução do algoritmo.
    pub fn observador<T>(&mut self, observador: T)
        where T: ObservadorEvolucao<Gene> + 'static
    {
        self.observador = Box::new(observador);
    }

    /// Busca pela solução para o algoritmo genético
    pub fn buscar_solucao(mut self) -> Gene {

        let mut pop = self.populacao;
        let observador = self.observador;
        let populacao_inicial = pop.individuos.len();

        // Notificando o observador que vamos iniciar a busca pela solução
        observador.inicio(&pop);

        let mut ger = 0;
        let mut mais_apto = pop.remover_mais_apto();

        // Executando até que o objetivo seja satisfeito
        while !self.objetivo.satisfeito_por(&mais_apto) {

            ger = ger + 1;
            pop.reiterar(mais_apto);

            // Evoluindo a população
            let mut nova_pop = self.evolucao.evoluir(&mut pop);

            // Aplicando elitismo quando informado.
            if self.elitismo {
                info!("Aplicando elitismo");
                nova_pop.reter(pop);
                pop = nova_pop
            } else {
                pop = nova_pop;
            }

            pop.manter_mais_aptos(populacao_inicial);

            // Informando ao observador sobre o avançar de uma geração
            observador.geracao(ger, &pop);
            mais_apto = pop.remover_mais_apto();
        }

        mais_apto.genes
    }
}


#[cfg(test)]
mod test {

    use super::*;
    use populacao::*;
    use evolucao::*;
    use objetivo::*;
    use aptidao::*;

    #[test]
    fn deve_achar_objetivo() {


        let mut pop = Populacao::criar_vazia(AptidaoSimples, Operacao::Max);
        pop.adicionar(1);
        pop.adicionar(2);
        pop.adicionar(3);
        pop.adicionar(4);

        let genetico = Genetico::criar(pop, EvolucaoSimples, ObjetivoSimples { valor: 6 });

        let resultado = genetico.buscar_solucao();
        assert_eq!(6, resultado);


    }

    struct ObjetivoSimples {
        valor: usize,
    }

    impl Objetivo<usize> for ObjetivoSimples {
        fn satisfeito_por(&mut self, ind: &Individuo<usize>) -> bool {
            self.valor == ind.genes
        }
    }

    /// Evolucao simples para testar o algoritmo
    struct EvolucaoSimples;

    impl Evolucao<usize> for EvolucaoSimples {
        fn evoluir(&self, pop: &mut Populacao<usize>) -> Populacao<usize> {

            let mut nova_pop = pop.preparar_nova_geracao();
            for ind in pop.individuos.iter() {
                nova_pop.adicionar(ind.genes + 1);
            }

            nova_pop
        }
    }


}
