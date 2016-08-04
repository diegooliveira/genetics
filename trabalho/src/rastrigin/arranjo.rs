

use configuracao::Configuracao;

use genetics::aptidao::Aptidao;
use genetics::evolucao::mutacao::Mutagenico;
use genetics::evolucao::cruzamento::*;
use genetics::aleatorio::Aleatorio;
use genetics::populacao::{CriadorIndividuos, Operacao};

/// Classe do problema
pub struct RastriginArranjo;

impl RastriginArranjo {
    pub fn format(&self, val: [f64; 2]) -> String {
        super::format(val[0], val[1])
    }

    pub fn processa(&self, _: &Configuracao) {}

    pub fn operacao(&self) -> Operacao {
        Operacao::Max
    }
}

/// Cruzador de genes para a função de Rastrigin
impl Cruzador<[f64; 2]> for RastriginArranjo {
    fn cruzar(&self,
              aleatorio: &mut Aleatorio,
              primeiro: &[f64; 2],
              segundo: &[f64; 2])
              -> ([f64; 2], [f64; 2]) {
        trace!("Cruzando Rastrigin");
        cruza_array_um_ponto!(aleatorio.chance(), 0.0, 2, primeiro, segundo)
    }
}

impl Aptidao<[f64; 2]> for RastriginArranjo {
    fn calcular_aptidao(&self, genes: &[f64; 2]) -> f64 {

        let x = genes[0];
        let y = genes[1];
        super::rastrigin(x, y)
    }
}

impl CriadorIndividuos<[f64; 2]> for RastriginArranjo {
    fn criar(&self, aleatorio: &mut Aleatorio) -> [f64; 2] {
        [aleatorio.intervalo(-5.0, 5.0), aleatorio.intervalo(-5.0, 5.0)]
    }
}

impl Mutagenico<[f64; 2]> for RastriginArranjo {
    fn mutar(&self, gene: &[f64; 2], aleatorio: &mut Aleatorio) -> [f64; 2] {

        let mut mutante = gene.clone();
        if aleatorio.chance() > 0.5 {
            mutante[0] = aleatorio.intervalo(-5.0, 5.0);
        } else {
            mutante[1] = aleatorio.intervalo(-5.0, 5.0);
        }
        mutante
    }
}
