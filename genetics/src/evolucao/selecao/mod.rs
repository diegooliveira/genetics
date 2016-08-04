

pub mod torneio;
pub mod roleta;

// Reexportando os tipos desse módulo para facilitar a vida.
pub use self::roleta::*;
pub use self::torneio::*;

use populacao::*;


/// Interface que define as várias estratégias de seleção de individuos para o
/// cruzamento.
pub trait Seletor<Gene> {
    /// Implementações desse método deverão selecionar dois individuos para
    /// cruzamento, removendo esses indivíduos da população
    fn seleciona(&self, pop: &mut Populacao<Gene>) -> Option<(Individuo<Gene>, Individuo<Gene>)>;
}
