

pub mod arranjo;
pub mod binario;

// Para facilitar a vida e não precisar ficar imporando a implementação com arranjo ou
// binária
pub use self::arranjo::*;
pub use self::binario::*;

// Trazendo para o contexto as funções de potencia, pi e cos
use std::f64;


pub fn format(x: f64, y: f64) -> String {
    let f = rastrigin(x, y);
    format!("x: {}, y: {}, resultado: {}", x, y, f)
}

pub fn rastrigin(x: f64, y: f64) -> f64 {
    let zx = x.powi(2) - 10.0 * (2.0 * f64::consts::PI * x).cos() + 10.0;
    let zy = y.powi(2) - 10.0 * (2.0 * f64::consts::PI * y).cos() + 10.0;

    let z = -(zx + zy);

    z
}
