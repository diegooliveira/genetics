
use populacao::Populacao;
use evolucao::*;
use aleatorio::Aleatorio;

use std::marker::PhantomData;


pub trait Mutagenico<Gene> {
    fn mutar(&self, gene: &Gene, aleatorio: &mut Aleatorio) -> Gene;
}

pub struct MutagenicoVetor {
    pub estados: Vec<usize>,
}

impl Mutagenico<Vec<usize>> for MutagenicoVetor {
    fn mutar(&self, gene: &Vec<usize>, aleatorio: &mut Aleatorio) -> Vec<usize> {

        let indice = aleatorio.intervalo(0, self.estados.len());
        let estado = aleatorio.intervalo(0, self.estados[indice]);

        let mut mutante = gene.clone();
        mutante[indice] = estado;

        mutante
    }
}



pub struct Mutacao<Mut, Gene>
    where Mut: Mutagenico<Gene>
{
    mutagenico: Mut,
    chance: f64,

    // Para satisfazer o compilador
    phanton: PhantomData<Gene>,
}

impl<Mut, Gene> Mutacao<Mut, Gene>
    where Mut: Mutagenico<Gene>
{
    pub fn criar(chance: f64, mutagenico: Mut) -> Self {
        Mutacao {
            chance: chance,
            mutagenico: mutagenico,
            phanton: PhantomData,
        }
    }
}


/// Implementação para quando a mutação é um operador usado na evolução do AG.
impl<Mut, Gene> Evolucao<Gene> for Mutacao<Mut, Gene>
    where Mut: Mutagenico<Gene>
{
    fn evoluir(&self, pop: &mut Populacao<Gene>) -> Populacao<Gene> {

        let mut mutantes = pop.preparar_nova_geracao();

        let mut aleatorio = Aleatorio::criar();
        let qtd = pop.individuos.len();
        for idx in 0..qtd {
            if self.chance > aleatorio.chance() {

                let mutante = self.mutagenico.mutar(&pop.individuos[idx].genes, &mut aleatorio);
                mutantes.adicionar(mutante);
            }
        }
        mutantes
    }
}
