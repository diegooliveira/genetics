
use populacao::*;

pub trait Objetivo<Gene> {
    fn satisfeito_por(&mut self, individuo: &Individuo<Gene>) -> bool;
}

/// O objetivo misto guarda uma relação de objetivos que se quer atingir.
pub struct ObjetivoMisto<Gene> {
    objetivos: Vec<Box<Objetivo<Gene>>>,
}

/// Implementação básica do objetivo misto.
impl<Gene> ObjetivoMisto<Gene> {
    /// Cria um novo objetivo misto vazio.
    pub fn criar() -> Self {
        ObjetivoMisto { objetivos: Vec::new() }
    }

    /// Adiciona um novo objetivo na lista de objetivos mistos.
    pub fn adicionar<Obj>(&mut self, objetivo: Obj)
        where Obj: Objetivo<Gene> + 'static
    {
        self.objetivos.push(Box::new(objetivo));
    }
}

/// Implementação do objetivo misto ao ser aplicado como objetivo na resolução de
/// um problema de AG.
impl<Gene> Objetivo<Gene> for ObjetivoMisto<Gene> {
    /// Avalia se algum dos objetovos controlados pelo objetivo misto foi
    /// satisfeito. Retorna assim true se qualquer um dos sobjetivos foi
    /// atingido e para de avalidar a lista de objetivos.
    fn satisfeito_por(&mut self, individuo: &Individuo<Gene>) -> bool {

        for obj in self.objetivos.iter_mut() {
            if obj.satisfeito_por(individuo) {
                return true;
            }
        }

        return false;
    }
}


pub struct NumeroMaximoIteracoes {
    maximo: usize,

    atual: usize,
}

impl NumeroMaximoIteracoes {
    pub fn criar(maximo: usize) -> Self {
        NumeroMaximoIteracoes {
            maximo: maximo,
            atual: 0,
        }
    }
}

impl<Gene> Objetivo<Gene> for NumeroMaximoIteracoes {
    fn satisfeito_por(&mut self, _: &Individuo<Gene>) -> bool {
        let eq = self.maximo == self.atual;

        self.atual = self.atual + 1;

        eq
    }
}

/// Estrutura que busca a aptidão entre um intervalo de valores.
pub struct ObjetivoIntervalo {
    /// A faixa superior exclusivo.
    max: f64,
    /// A faixa inferior exclusivo.
    min: f64,
}

impl ObjetivoIntervalo {
    /// Cria uma nova instância com valores de intervalo mínimo e máximo.
    pub fn criar(min: f64, max: f64) -> Self {
        ObjetivoIntervalo {
            min: min,
            max: max,
        }
    }
}

/// Implementação do ObjetivoIntervalo como um objetivo.
impl<Gene> Objetivo<Gene> for ObjetivoIntervalo {
    /// Avalia se a aptidão do indivíduo de melhor aptidao está dentro da faixa procurada.
    fn satisfeito_por(&mut self, ind: &Individuo<Gene>) -> bool {

        self.min.lt(&ind.aptidao) && self.max.gt(&ind.aptidao)

    }
}




#[cfg(test)]
mod test {

    use super::*;
    use populacao::Individuo;

    #[test]
    fn deve_respeitar_repeticao_maxima() {
        let mut nmi = NumeroMaximoIteracoes::criar(4);
        let mut qtd = 0;
        while !nmi.satisfeito_por(&Individuo {
            genes: 10,
            aptidao: 1.0,
        }) {
            qtd = qtd + 1;
        }
        assert_eq!(4, qtd);
    }

    #[test]
    fn deve_parar_primeiro_objetivo_multiplo_satisfeito() {
        let mut objetivo = ObjetivoMisto::criar();
        objetivo.adicionar(ObjetivoTeste {
            valor: true,
            chamado: false,
        });
        objetivo.adicionar(ObjetivoTeste {
            valor: true,
            chamado: false,
        });
        objetivo.adicionar(ObjetivoTeste {
            valor: true,
            chamado: false,
        });

        let res = objetivo.satisfeito_por(&Individuo {
            genes: 10,
            aptidao: 1.0,
        });

        assert_eq!(true, res);
    }

    #[test]
    fn deve_validar_objetivos_multiplos() {

        let mut objetivo = ObjetivoMisto::criar();
        objetivo.adicionar(ObjetivoTeste {
            valor: false,
            chamado: false,
        });
        objetivo.adicionar(ObjetivoTeste {
            valor: false,
            chamado: false,
        });
        objetivo.adicionar(ObjetivoTeste {
            valor: false,
            chamado: false,
        });

        let res = objetivo.satisfeito_por(&Individuo {
            genes: 10,
            aptidao: 1.0,
        });
        assert_eq!(false, res);

    }

    struct ObjetivoTeste {
        valor: bool,
        chamado: bool,
    }

    impl Objetivo<usize> for ObjetivoTeste {
        fn satisfeito_por(&mut self, _: &Individuo<usize>) -> bool {
            self.chamado = true;

            return self.valor;
        }
    }



}
