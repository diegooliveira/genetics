

pub trait Aptidao<Gene> {
    fn calcular_aptidao(&self, genes: &Gene) -> f64;
}


/// Aptidão simples usada para testes. Nessa aptidao o próprio gene é o valor da aptidão.
#[derive(Clone)]
pub struct AptidaoSimples;

impl Aptidao<usize> for AptidaoSimples {
    fn calcular_aptidao(&self, genes: &usize) -> f64 {
        *genes as f64
    }
}
