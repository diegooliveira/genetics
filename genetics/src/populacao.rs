//! # População
//!
//! Módulo com lógica de utilização da população no Algoritmo Genético
//!

use std::rc::Rc;
use std::cmp::*;

use aptidao::*;
use aleatorio::*;

pub struct Individuo<T> {
    pub genes: T,
    pub aptidao: f64,
}

/// Interface usada para criar indivíduos
pub trait CriadorIndividuos<T> {
    /// Cria um indivíduo aletório. A insntância de aleatório está aqui para ajudar a
    /// implementação na geração de numeros aleatórios.
    fn criar(&self, aleatorio: &mut Aleatorio) -> T;
}

/// Enumerador que descreve o motivo de existencia da população
#[derive(Clone)]
pub enum Operacao {
    /// Utilizado para fazer a minimizacao
    Min,

    /// Usado para pegar a maximizacao
    Max,
}

impl Operacao {
    /// Retorna Ordering::Greater se *a* tem melhror valor de aptidão que *b* para o tipo
    /// de operacao
    /// Maximizacao
    ///     a = 10, b = 8  => Ordering::Greater
    ///     a = 10, b = 11 => Ordering::Less
    /// Minimizacao
    ///     a = 10, b = 8  => Ordering::Less
    ///     a = 10, b = 11 => Ordering::Greater
    pub fn compara(&self, a: &f64, b: &f64) -> Ordering {
        match self {
            &Operacao::Max => a.partial_cmp(&b).unwrap(),
            &Operacao::Min => b.partial_cmp(&a).unwrap(),
        }
    }

    /// Método auxiliar para petar o mais apto dentro de dois possívels valores de aptidão
    pub fn mais_apto(&self, a: f64, b: f64) -> f64 {
        if self.compara(&a, &b) == Ordering::Greater {
            a
        } else {
            b
        }
    }

    /// Metod auxiliar para pegar o menos apto dentro dois valores de aptidão.
    pub fn menos_apto(&self, a: f64, b: f64) -> f64 {
        if self.compara(&a, &b) == Ordering::Less {
            a
        } else {
            b
        }
    }
}

/// População
pub struct Populacao<T> {
    /// Referencia para uma implementação que consegue calcular a aptidão para um tipo T
    pub aptidao: Rc<Aptidao<T>>,
    /// A lista de indivíduos dessa populaçao
    pub individuos: Vec<Individuo<T>>,
    /// O tipo de operacão dessa população
    operacao: Operacao,
}

/// Implementação dos métodos da população
impl<Gene> Populacao<Gene> {
    /// Cria uma nova população sem nenhum indivíduo
    pub fn criar_vazia<Apt>(aptidao: Apt, operacao: Operacao) -> Self
        where Apt: Aptidao<Gene> + 'static
    {
        Populacao {
            aptidao: Rc::new(aptidao),
            individuos: Vec::new(),
            operacao: operacao,
        }
    }

    /// Cria uma população com um conjunto de indivíduos.
    pub fn criar<'a, Apt, CIA>(aptidao: Apt,
                               criador: &CIA,
                               quantidade: usize,
                               operacao: Operacao)
                               -> Self
        where Apt: Aptidao<Gene> + 'static,
              CIA: CriadorIndividuos<Gene>
    {
        debug!("Iniciando população");
        let mut aleatorio = Aleatorio::criar();
        let mut individuos = Vec::new();
        for _ in 0..quantidade {
            let genes = criador.criar(&mut aleatorio);
            let apt = aptidao.calcular_aptidao(&genes);
            let ind = Individuo {
                genes: genes,
                aptidao: apt,
            };
            individuos.push(ind);
        }

        trace!("População inicial criada");
        Populacao {
            aptidao: Rc::new(aptidao),
            individuos: individuos,
            operacao: operacao,
        }
    }

    /// Busca na populacao o individuo mais apto.
    pub fn remover_mais_apto(&mut self) -> Individuo<Gene> {

        // Pegando o index do elemento mais apto dessa geração
        let mut idx = 0;
        {
            let mut maior = &self.individuos[0];
            for i in 1..self.individuos.len() {
                let outro = &self.individuos[i];
                let comp = self.operacao.compara(&maior.aptidao, &outro.aptidao);

                if comp == Ordering::Less {
                    maior = outro;
                    idx = i;
                }
            }
        }

        self.individuos.remove(idx)
    }

    pub fn adicionar(&mut self, genes: Gene) {

        let aptidao = self.aptidao.calcular_aptidao(&genes);

        self.individuos.push(Individuo {
            aptidao: aptidao,
            genes: genes,
        });

    }

    pub fn manter_mais_aptos(&mut self, quantiade: usize) {

        let ref operacao = self.operacao;

        self.individuos.sort_by(|a, b| operacao.compara(&a.aptidao, &b.aptidao).reverse());
        let tamanho_atual = self.individuos.len();

        for i in (quantiade..tamanho_atual).rev() {
            self.individuos.remove(i);
        }

    }

    /// Devolve para a população o individuo mais apto.
    pub fn reiterar(&mut self, ind: Individuo<Gene>) {
        self.individuos.push(ind);
    }

    pub fn reter(&mut self, outra: Populacao<Gene>) {
        let mut inds = outra.individuos;
        self.individuos.append(&mut inds);
    }

    pub fn preparar_nova_geracao(&self) -> Self {
        Populacao {
            aptidao: self.aptidao.clone(),
            individuos: Vec::new(),
            operacao: self.operacao.clone(),
        }
    }

    pub fn aptidao_media(&self) -> f64 {
        let total = self.individuos.iter().map(|i| i.aptidao).fold(0.0, |a, b| a + b) as f64;
        let qtd = self.individuos.len() as f64;

        total / qtd
    }

    pub fn melhor_aptidao(&self) -> f64 {
        let maior = self.individuos[0].aptidao;
        self.individuos.iter().map(|i| i.aptidao).fold(maior, |a, b| self.operacao.mais_apto(a, b))
    }

    pub fn pior_aptidao(&self) -> f64 {
        let menor = self.individuos[0].aptidao;
        self.individuos.iter().map(|i| i.aptidao).fold(menor, |a, b| self.operacao.menos_apto(a, b))
    }

    pub fn desvio_aptidao(&self) -> f64 {

        let media = self.aptidao_media();

        let soma_quadado_diff = self.individuos
                                    .iter()
                                    .map(|i| (i.aptidao - media).powi(2))
                                    .fold(0.0, |a, b| a + b);

        soma_quadado_diff / (self.individuos.len() as f64 - 1.0)
    }
}


pub fn cria_populacao(entrada: &Vec<usize>, oper: Operacao) -> Populacao<usize> {
    let mut pop = Populacao::criar_vazia(AptidaoSimples, oper);
    for val in entrada.iter() {
        pop.adicionar(*val);
    }
    pop
}

#[cfg(test)]
mod test {
    //! Módulo de testes para as implementações feitas.
    use std::cmp::Ordering;

    use super::*;

    /// / Valida se a aptidao média está sendo calculada corretamente
    #[test]
    fn deve_pegar_aptidao_media() {
        let populacao = cria_populacao(&vec![0, 1, 2, 3, 4], Operacao::Max);
        assert_eq!(populacao.aptidao_media(), 2.0);
    }

    #[test]
    fn deve_achar_melhor_aptidao() {
        let aptidoes = vec![0, 1, 2, 3, 4];
        assert_eq!(4.0,
                   cria_populacao(&aptidoes, Operacao::Max).melhor_aptidao());
        assert_eq!(0.0,
                   cria_populacao(&aptidoes, Operacao::Min).melhor_aptidao());
    }

    /// Validando se a população recupera o valor da aptidão correta de acordo
    /// com a operação daquela população
    #[test]
    fn deve_achar_pior_aptidao() {

        let aptidoes = vec![0, 1, 2, 3, 4];
        // Quando estamos maximizando o pior valor de fitness é a pior aptidao
        assert_eq!(0.0, cria_populacao(&aptidoes, Operacao::Max).pior_aptidao());

        // Quando estamos minimizando, o melhor valor de fitness é a pior aptidão
        assert_eq!(4.0, cria_populacao(&aptidoes, Operacao::Min).pior_aptidao());
    }

    /// Verificando que a população separa o indivíduo mais apto de acordo com
    /// com a operação daquela população
    #[test]
    fn deve_encontrar_o_individuo_mais_apto() {
        valida_individuo_mais_apto(Operacao::Max, vec![1, 2, 3, 4], 4);
        valida_individuo_mais_apto(Operacao::Min, vec![1, 2, 3, 4], 1);
    }

    #[test]
    fn deve_mandater_individuos_mais_aptos() {
        valida_mais_apto(Operacao::Max, vec![1, 2, 3, 4], 2, vec![4, 3]);
        valida_mais_apto(Operacao::Min, vec![1, 2, 3, 4], 2, vec![1, 2]);
    }

    #[test]
    fn deve_comparar_correto() {

        assert_eq!(Operacao::Max.compara(&10.0, &8.0), Ordering::Greater);
        assert_eq!(Operacao::Max.compara(&8.0, &10.0), Ordering::Less);

        assert_eq!(Operacao::Min.compara(&10.0, &8.0), Ordering::Less);
        assert_eq!(Operacao::Min.compara(&8.0, &10.0), Ordering::Greater);
    }

    fn valida_individuo_mais_apto(op: Operacao, valores: Vec<usize>, esperado: usize) {

        let mut pop = cria_populacao(&valores, op);

        let mais_apto = pop.remover_mais_apto();
        assert_eq!(esperado, mais_apto.genes);
    }

    fn valida_mais_apto(oper: Operacao, entrada: Vec<usize>, qtd: usize, saida: Vec<usize>) {

        let mut pop = cria_populacao(&entrada, oper);
        pop.manter_mais_aptos(qtd);
        assert_eq!(pop.individuos.len(), qtd);
        for (idx, val) in pop.individuos.iter().enumerate() {
            assert_eq!(val.genes, saida[idx]);
        }
    }
}
