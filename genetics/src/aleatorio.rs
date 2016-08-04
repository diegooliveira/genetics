
use rand::{thread_rng, Rng, ThreadRng};
use rand::distributions::range::SampleRange;
use std::fmt::Debug;

pub struct Aleatorio {
    rand: ThreadRng,
    vicio: Option<f64>,
}

impl Aleatorio {
    pub fn criar() -> Self {
        Aleatorio {
            rand: thread_rng(),
            vicio: None,
        }
    }

    pub fn viciado(valor: f64) -> Self {
        Aleatorio {
            rand: thread_rng(),
            vicio: Some(valor),
        }
    }


    pub fn intervalo<T: PartialOrd + SampleRange + Debug>(&mut self, inicio: T, fim: T) -> T {
        self.rand.gen_range(inicio, fim)
    }

    pub fn chance(&mut self) -> f64 {
        self.vicio.unwrap_or_else(|| self.rand.gen())
    }
}
