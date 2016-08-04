
use configuracao::Configuracao;

use genetics::aptidao::Aptidao;
use genetics::evolucao::mutacao::Mutagenico;
use genetics::evolucao::cruzamento::*;
use genetics::aleatorio::Aleatorio;
use genetics::populacao::{CriadorIndividuos, Operacao};

/// Classe do problema
pub struct MultimodalBinario;

pub fn map(valores: &[bool]) -> Vec<i32> {
    let mut valores_mapeados = Vec::new();
    let mut valor = 0;
    for (idx, v) in valores.iter().enumerate() {

        valor = valor << 1;
        if *v {
            valor = valor | 0b1;
        }

        let resto = (idx + 1) % 10;
        if resto == 0 {
            let valor_processado = (-500.0 + (valor as f64 / 1023.0) * 1000.0) as i32;

            valores_mapeados.push(valor_processado);
            valor = 0;
        }
    }

    valores_mapeados
}


impl MultimodalBinario {
    pub fn format(&self, genes: [bool; 300]) -> String {

        let valores = map(&genes);
        let mut texto = String::with_capacity(300);
        let valor = format!("f: {}", super::aptidao_unimodal(&*valores));

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
impl Aptidao<[bool; 300]> for MultimodalBinario {
    fn calcular_aptidao(&self, genes: &[bool; 300]) -> f64 {
        let valores = map(genes);
        super::aptidao_unimodal(&*valores)
    }
}

impl CriadorIndividuos<[bool; 300]> for MultimodalBinario {
    fn criar(&self, aleatorio: &mut Aleatorio) -> [bool; 300] {
        let mut genes = [false; 300];
        for idx in 0..300 {
            genes[idx] = aleatorio.chance() > 0.5;
        }
        genes
    }
}

/// Cruzador de genes para a função de Rastrigin
impl Cruzador<[bool; 300]> for MultimodalBinario {
    fn cruzar(&self,
              aleatorio: &mut Aleatorio,
              primeiro: &[bool; 300],
              segundo: &[bool; 300])
              -> ([bool; 300], [bool; 300]) {
        trace!("Cruzando UnimodalArranjo");
        let idx_sorte = (aleatorio.chance() * 300.0) as usize;
        let mut a = [false; 300];
        let mut b = [false; 300];

        for idx in 0..300 {

            if idx < idx_sorte {
                a[idx] = primeiro[idx];
                b[idx] = segundo[idx];
            } else {
                a[idx] = segundo[idx];
                b[idx] = primeiro[idx];
            }
        }

        (a, b)
    }
}

impl Mutagenico<[bool; 300]> for MultimodalBinario {
    fn mutar(&self, gene: &[bool; 300], aleatorio: &mut Aleatorio) -> [bool; 300] {


        let mut mutante = [false; 300];
        for idx in 0..300 {
            mutante[idx] = gene[idx];
        }

        let indice = aleatorio.intervalo(0, 300);
        mutante[indice] = aleatorio.chance() > 0.5;

        mutante
    }
}

#[cfg(test)]
mod test {


    use super::*;


    #[test]
    fn deve_mapear() {

        let vals = map(&[//
                         true,
                         true,
                         true,
                         true,
                         true,
                         true,
                         true,
                         true,
                         true,
                         true,
                         false,
                         false,
                         false,
                         false,
                         false,
                         false,
                         false,
                         false,
                         false,
                         false]);
        assert_eq!(500, vals[0]);
        assert_eq!(-500, vals[1]);
    }
}
