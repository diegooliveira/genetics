
pub mod arranjo;
pub mod binario;

pub use self::arranjo::*;
pub use self::binario::*;

pub fn aptidao_unimodal(genes: &[i32]) -> f64 {
    let soma_quadrados = genes.iter()
                              .map(|v| *v as f64)
                              .map(|v| -v * v.abs().sqrt().sin())
                              .map(|v| v * v)
                              .fold(0.0, |a, b| a + b);

    soma_quadrados
}
