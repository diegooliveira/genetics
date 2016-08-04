

use populacao::*;
use evolucao::selecao::Seletor;

use aleatorio::Aleatorio;

pub struct SeletorPorRoleta;

/// Implementação de seleção de indivíduos usando o método de roleta.
impl SeletorPorRoleta {
    /// Cria uma nova instância
    pub fn criar() -> Self {
        SeletorPorRoleta
    }
}

/// Estrutura interna auxiliar usada na hora de "rodar a roleta"
#[derive(Debug)]
pub struct Roleta {
    /// Indice do indivíduo
    idx: usize,
    /// Inicio da chance de ser sorteado
    faixa_inicio: f64,
    /// Fim da chance de ser sorteado.
    faixa_fim: f64,
}

/// Implementação da seleção de indivíduos por roleta quando ele é usado como seletor.
impl<Gene> Seletor<Gene> for SeletorPorRoleta {
    fn seleciona(&self, pop: &mut Populacao<Gene>) -> Option<(Individuo<Gene>, Individuo<Gene>)> {



        // Encontra os dois indivíduos que devem ser cruzados.
        trace!("Executando seleção por roleta");
        let (posicao_primeiro, mut posicao_segundo) = encontra_par(pop);


        trace!("Selecionado para cruzamento {} {}",
               posicao_primeiro,
               posicao_segundo);
        let primeiro = pop.individuos.remove(posicao_primeiro);

        if posicao_primeiro < posicao_segundo {
            posicao_segundo = posicao_segundo - 1;
        }

        if posicao_segundo == posicao_primeiro {
            if posicao_segundo == 0 {
                posicao_segundo = 1
            } else {
                posicao_segundo -= 1;
            }
        }
        let segundo = pop.individuos.remove(posicao_segundo);

        Some((primeiro, segundo))
    }
}


fn encontra_par<Gene>(pop: &Populacao<Gene>) -> (usize, usize) {


    let roleta = gera_roleta(&pop);

    let mut aleatorio = Aleatorio::criar();
    let chance_primeiro = aleatorio.intervalo(0.0, 1.0);
    let chance_segundo = aleatorio.intervalo(0.0, 1.0);

    let posicao_primeiro = recupera_posicao(&roleta, chance_primeiro);
    let posicao_segundo = recupera_posicao(&roleta, chance_segundo);

    (posicao_primeiro, posicao_segundo)
}

pub fn recupera_posicao(roleta: &Vec<Roleta>, chance: f64) -> usize {
    roleta.iter()
          .find(|roleta| roleta.faixa_inicio <= chance && roleta.faixa_fim > chance)
          .map(|roleta| roleta.idx)
          .unwrap()
}

/// Gera a roleta com base no aptidão da população
pub fn gera_roleta<Gene>(pop: &Populacao<Gene>) -> Vec<Roleta> {

    // Encontra o valor toda da aptidão dos indivíduos para distribuir na roleta
    let aptidao_total = pop.individuos
                           .iter()
                           .map(|ind| ind.aptidao)
                           .fold(0.0, |a, b| a + b);
    trace!("Aptidão total: {}", aptidao_total);

    // Monta a roleta
    let mut roleta = Vec::new();
    let mut inicio = 0.0;
    for (idx, ind) in pop.individuos.iter().enumerate() {
        let participacao = inicio + ind.aptidao / aptidao_total;
        roleta.push(Roleta {
            idx: idx,
            faixa_inicio: inicio,
            faixa_fim: participacao,
        });
        inicio = participacao;
    }

    trace!("Roleta: {:?}", roleta);

    roleta
}


#[cfg(test)]
mod test {

    use super::*;
    use populacao::*;

    const ERRO_ACEITO: f64 = 0.00000001;

    fn gera_roleta_teste() -> Vec<Roleta> {
        let populacao = cria_populacao(&vec![0, 1, 2, 3, 4], Operacao::Max);
        gera_roleta(&populacao)
    }

    #[test]
    fn deve_calcular_roleta() {
        let roleta = gera_roleta_teste();
        assert_eq!(roleta.len(), 5);

        assert_eq!(roleta[0].idx, 0);
        assert_eq!(roleta[0].faixa_inicio, 0.0);
        assert_eq!(roleta[0].faixa_fim, 0.0);

        assert_eq!(roleta[1].idx, 1);
        assert!(roleta[1].faixa_inicio - 0.0 < ERRO_ACEITO);
        assert!(roleta[1].faixa_fim - 0.1 < ERRO_ACEITO);

        assert_eq!(roleta[2].idx, 2);
        assert!(roleta[2].faixa_inicio - 0.1 < ERRO_ACEITO);
        assert!(roleta[2].faixa_fim - 0.3 < ERRO_ACEITO);

        assert_eq!(roleta[3].idx, 3);
        assert!(roleta[3].faixa_inicio - 0.3 < ERRO_ACEITO);
        assert!(roleta[3].faixa_fim - 0.6 < ERRO_ACEITO);

        assert_eq!(roleta[4].idx, 4);
        assert!(roleta[4].faixa_inicio - 0.6 < ERRO_ACEITO);
        assert!(roleta[4].faixa_fim - 1.0 < ERRO_ACEITO);
    }

    #[test]
    fn deve_recuperar_posicao_correta() {
        let roleta = gera_roleta_teste();
        let indice = recupera_posicao(&roleta, 0.5);

        assert_eq!(indice, 3);
    }

}
