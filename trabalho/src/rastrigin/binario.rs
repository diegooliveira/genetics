
use configuracao::Configuracao;

use genetics::aptidao::Aptidao;
use genetics::evolucao::mutacao::Mutagenico;
use genetics::evolucao::cruzamento::*;
use genetics::aleatorio::Aleatorio;
use genetics::populacao::{CriadorIndividuos, Operacao};

use std::u16;

/// Classe do problema
pub struct RastriginBinario;

impl RastriginBinario {
    pub fn format(&self, genes: u32) -> String {

        let (x, y) = divide(&genes);
        super::format(x, y)
    }
    pub fn processa(&self, _: &Configuracao) {}


    pub fn operacao(&self) -> Operacao {
        Operacao::Max
    }
}

/// Cruzador de genes para a função de Rastrigin. Estou usando alguns deslocamentos
/// de bits para função ficar mais rápida.
impl Cruzador<u32> for RastriginBinario {
    fn cruzar(&self, aleatorio: &mut Aleatorio, primeiro: &u32, segundo: &u32) -> (u32, u32) {

        let tamanho = (aleatorio.chance() * 32.0) as u32;
        let mascara = !0u32 << tamanho;

        let f1 = (mascara & primeiro) | (!mascara & segundo);
        let f2 = (!mascara & primeiro) | (mascara & segundo);
        (f1, f2)
    }
}



impl Aptidao<u32> for RastriginBinario {
    fn calcular_aptidao(&self, genes: &u32) -> f64 {


        let (x, y) = divide(genes);
        super::rastrigin(x, y)
    }
}

use std::u32;

impl CriadorIndividuos<u32> for RastriginBinario {
    fn criar(&self, aleatorio: &mut Aleatorio) -> u32 {
        let val = aleatorio.intervalo(0, u32::MAX) as u32;

        val
    }
}

impl Mutagenico<u32> for RastriginBinario {
    fn mutar(&self, gene: &u32, aleatorio: &mut Aleatorio) -> u32 {


        trocar_bit(*gene, aleatorio.intervalo(0, 32))
    }
}

/// Função que converte de
pub fn binario_para_decimal(valor: u16) -> f64 {

    let val = -5.0 + (valor as f64 / u16::MAX as f64) * 10.0;

    val
}

fn divide(genes: &u32) -> (f64, f64) {
    let x = binario_para_decimal(((genes & 0xFFFF0000) >> 16) as u16);
    let y = binario_para_decimal((genes & 0x0000FFFF) as u16);

    (x, y)
}

pub fn trocar_bit(valor: u32, posicao: usize) -> u32 {
    valor ^ (1 << posicao)
}

#[cfg(test)]
mod test {

    use super::*;
    use genetics::evolucao::cruzamento::Cruzador;
    use genetics::aleatorio::Aleatorio;

    #[test]
    fn cruzamento_binario() {
        let cruz = RastriginBinario;

        let pri: u32 = 0b0101010101010111_0011011101010111; // x1_y1
        let seg: u32 = 0b1101010110011101_0001111101110011; // x2_y2

        let mut aleatorio = Aleatorio::viciado(0.5);
        let (a, b) = cruz.cruzar(&mut aleatorio, &pri, &seg);

        assert_eq!(a, 0b0101010101010111_0001111101110011); // x1_y2
        assert_eq!(b, 0b1101010110011101_0011011101010111); // x2_y1
    }

    #[test]
    fn troca_bit() {

        let exemp: u32 = 0b10010010001001010100100_1_01110010;
        let esper: u32 = 0b10010010001001010100100_0_01110010;

        let retorno = trocar_bit(exemp, 8);
        println!("Valores de teste:\n\tEntrada: {:b}\n\t  Saida: {:b}",
                 exemp,
                 retorno);

        assert_eq!(retorno, esper);
    }

    #[test]
    fn conversao_binario_decimal() {
        assert_eq!(binario_para_decimal(0), -5.0);
        assert_eq!(binario_para_decimal(32768).round(), 0.0);
        assert_eq!(binario_para_decimal(65535), 5.0);
    }

}
