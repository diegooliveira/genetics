
use populacao::*;

/// Interface que permite observar o processo de evolução dos indivíduos no AG
pub trait ObservadorEvolucao<Gene> {
    /// Método chamado no inicio da busca pela solução.
    fn inicio(&self, pop: &Populacao<Gene>);
    fn geracao(&self, geracao: usize, pop: &Populacao<Gene>);
}

pub struct ObservadorEvolucaoVazio;

impl<Gene> ObservadorEvolucao<Gene> for ObservadorEvolucaoVazio {
    fn inicio(&self, _: &Populacao<Gene>) {}
    fn geracao(&self, _: usize, _: &Populacao<Gene>) {}
}

pub struct ObservadorEvolucaoImprimeAptidao;

impl<Gene> ObservadorEvolucao<Gene> for ObservadorEvolucaoImprimeAptidao {
    fn inicio(&self, pop: &Populacao<Gene>) {
        println!("Geracao;Maior;Menor;Media;Desvio");
        println!("0;{};{};{};{}",
                 pop.melhor_aptidao(),
                 pop.pior_aptidao(),
                 pop.aptidao_media(),
                 pop.desvio_aptidao());
    }
    fn geracao(&self, geracao: usize, pop: &Populacao<Gene>) {
        println!("{};{};{};{};{}",
                 geracao,
                 pop.melhor_aptidao(),
                 pop.pior_aptidao(),
                 pop.aptidao_media(),
                 pop.desvio_aptidao());
    }
}
