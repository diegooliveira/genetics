
use configuracao::Configuracao;

use genetics::aptidao::Aptidao;
use genetics::evolucao::mutacao::Mutagenico;
use genetics::evolucao::cruzamento::*;
use genetics::aleatorio::Aleatorio;
use genetics::populacao::{CriadorIndividuos, Operacao};

/// Classe do problema
pub struct MultimodalArranjo;



impl MultimodalArranjo {
    pub fn format(&self, genes: [i32; 30]) -> String {
        let mut texto = String::with_capacity(300);

        let valor = format!("f: {}", super::aptidao_unimodal(&genes));

        texto.push_str(&*valor);
        for (idx, gene) in genes.iter().enumerate() {
            let valorx = format!(" x_{}: {},", idx, gene);
            texto.push_str(&*valorx);
        }

        texto
    }

    pub fn processa(&self, _: &Configuracao) {}

    pub fn operacao(&self) -> Operacao {
        Operacao::Min
    }
}

/// Implementação que calcula a aptidão dos valores para o arranjo unimodal
impl Aptidao<[i32; 30]> for MultimodalArranjo {
    fn calcular_aptidao(&self, genes: &[i32; 30]) -> f64 {
        super::aptidao_unimodal(genes)
    }
}

impl CriadorIndividuos<[i32; 30]> for MultimodalArranjo {
    fn criar(&self, aleatorio: &mut Aleatorio) -> [i32; 30] {
        let mut genes = [0i32; 30];
        for idx in 0..30 {
            genes[idx] = aleatorio.intervalo(-500, 500);
        }
        genes
    }
}

/// Cruzador de genes para a função de Rastrigin
impl Cruzador<[i32; 30]> for MultimodalArranjo {
    fn cruzar(&self,
              aleatorio: &mut Aleatorio,
              primeiro: &[i32; 30],
              segundo: &[i32; 30])
              -> ([i32; 30], [i32; 30]) {
        trace!("Cruzando UnimodalArranjo");
        cruza_array_um_ponto!(aleatorio.chance(), 0, 30, primeiro, segundo)
    }
}

impl Mutagenico<[i32; 30]> for MultimodalArranjo {
    fn mutar(&self, gene: &[i32; 30], aleatorio: &mut Aleatorio) -> [i32; 30] {

        let mut mutante = gene.clone();
        let indice = aleatorio.intervalo(0, 30);
        mutante[indice] = aleatorio.intervalo(-100, 100);

        mutante
    }
}
