
use configuracao::Configuracao;

use genetics::aptidao::Aptidao;
use genetics::evolucao::mutacao::Mutagenico;
use genetics::evolucao::cruzamento::*;
use genetics::aleatorio::Aleatorio;
use genetics::populacao::{CriadorIndividuos, Operacao};


/// Classe do problema
pub struct UnimodalArranjoDois {
    cruzamento_um_ponto: bool,
}

impl UnimodalArranjoDois {
    pub fn criar() -> Self {
        UnimodalArranjoDois { cruzamento_um_ponto: true }
    }
}

fn aptidao_unimodal(genes: &[i32; 30]) -> f64 {
    let soma_quadrados = genes.iter()
                              .map(|v| *v as f64)
                              .map(|v| (v + 0.5).abs())
                              .map(|v| v * v)
                              .fold(0.0, |a, b| a + b);

    soma_quadrados
}

impl UnimodalArranjoDois {
    pub fn format(&self, genes: [i32; 30]) -> String {
        let mut texto = String::with_capacity(300);
        let valor = format!("f: {}", aptidao_unimodal(&genes));

        texto.push_str(&*valor);
        for (idx, gene) in genes.iter().enumerate() {
            let valorx = format!(" x_{}: {},", idx, gene);
            texto.push_str(&*valorx);
        }

        texto
    }

    pub fn processa(&mut self, conf: &Configuracao) {
        self.cruzamento_um_ponto = conf.modificador_cruzamento == "um-ponto";
    }

    pub fn operacao(&self) -> Operacao {
        Operacao::Min
    }
}

/// Implementação que calcula a aptidão dos valores para o arranjo unimodal
impl Aptidao<[i32; 30]> for UnimodalArranjoDois {
    fn calcular_aptidao(&self, genes: &[i32; 30]) -> f64 {
        aptidao_unimodal(genes)
    }
}

impl CriadorIndividuos<[i32; 30]> for UnimodalArranjoDois {
    fn criar(&self, aleatorio: &mut Aleatorio) -> [i32; 30] {
        let mut genes = [0i32; 30];
        for idx in 0..30 {
            genes[idx] = aleatorio.intervalo(-100, 100);
        }
        genes
    }
}

/// Cruzador de genes para a função de Rastrigin
impl Cruzador<[i32; 30]> for UnimodalArranjoDois {
    fn cruzar(&self,
              aleatorio: &mut Aleatorio,
              primeiro: &[i32; 30],
              segundo: &[i32; 30])
              -> ([i32; 30], [i32; 30]) {
        trace!("Cruzando UnimodalArranjo");
        if self.cruzamento_um_ponto {
            cruza_array_um_ponto!(aleatorio.chance(), 0, 30, primeiro, segundo)
        } else {
            cruza_array_dois_pontos!(aleatorio.chance(),
                                     aleatorio.chance(),
                                     0,
                                     30,
                                     primeiro,
                                     segundo)
        }
    }
}

impl Mutagenico<[i32; 30]> for UnimodalArranjoDois {
    fn mutar(&self, gene: &[i32; 30], aleatorio: &mut Aleatorio) -> [i32; 30] {

        let mut mutante = gene.clone();
        let indice = aleatorio.intervalo(0, 30);
        mutante[indice] = aleatorio.intervalo(-100, 100);

        mutante
    }
}
