
use populacao::*;
use evolucao::selecao::Seletor;
use aleatorio::Aleatorio;


/// Classe que faz seleção de indivíduos para cruzamento usando a técnica de torneio.
pub struct SelecaoPorTorneio {
    participantes: usize, // Quantidade de perticipantes no torneio
}


impl SelecaoPorTorneio {
    pub fn criar(participantes: usize) -> Self {
        SelecaoPorTorneio { participantes: participantes }
    }
}

impl<Gene> Seletor<Gene> for SelecaoPorTorneio {
    fn seleciona(&self, pop: &mut Populacao<Gene>) -> Option<(Individuo<Gene>, Individuo<Gene>)> {

        trace!("Executando seleção por torneio");
        if pop.individuos.len() < 2 {
            return None;
        }

        if pop.individuos.len() < self.participantes {
            panic!("Quantidade de individuos({}) menor que o torneio({})",
                   pop.individuos.len(),
                   self.participantes);
        }

        let mut torneio = pop.preparar_nova_geracao();
        let mut aleatorio = Aleatorio::criar();
        for _ in 0..self.participantes {
            let idx = aleatorio.intervalo(0, pop.individuos.len());

            let individuo = pop.individuos.remove(idx);

            torneio.reiterar(individuo);
        }

        let primeiro = torneio.remover_mais_apto();
        let segundo = torneio.remover_mais_apto();

        pop.individuos.append(&mut torneio.individuos);

        Some((primeiro, segundo))
    }
}
