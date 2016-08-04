extern crate genetics;
extern crate rand;

use genetics::populacao::{Populacao, Operacao};
use genetics::aptidao::Aptidao;
use genetics::evolucao::*;
use genetics::objetivo::*;
use genetics::evolucao::cruzamento::*;
use genetics::evolucao::mutacao::*;
use genetics::genetico::*;
use genetics::evolucao::selecao::torneio::*;

use rand::{thread_rng};
use rand::distributions::{IndependentSample, Range};

const TEMPO_MAXIMO: usize = 20;
const QTD_TAREFAS: usize = 20;
const QTD_MAQUINAS: usize = 30;
const QTD_AMOSTRA: usize = 100;

fn main() {
	
	let tempos = criar_tempos_aleatorios(QTD_TAREFAS, TEMPO_MAXIMO);
	let aptidao = AptidaoTarefasMaquinas{
    		qtd_maquinas:  QTD_MAQUINAS,
    		tempo_task: tempos.clone() 
    };
	
    // A população que será trabalhada
    let mut pop = Populacao::criar_vazia(aptidao, Operacao::Max);
    preencher_com_posicoes_aleatorias(&mut pop, QTD_MAQUINAS, QTD_AMOSTRA, QTD_TAREFAS);

	let estados = (0..QTD_TAREFAS).map(|_| QTD_MAQUINAS).collect::<Vec<usize>>();

    // Preparando o modelo de evolução que estamos interessado
    let mut evolucao = EvolucaoMista::criar();
    evolucao.adicionar(Mutacao::criar(0.4, MutagenicoVetor{
    			estados: estados
    }));
    evolucao.adicionar(Cruzamento::criar(SelecaoPorTorneio::criar(5),
                                         CruzadorVetor,
                                         0.9));


    // Configurando os objetivos do algoritmo
    let mut objetivo = ObjetivoMisto::criar();
    objetivo.adicionar(NumeroMaximoIteracoes::criar(10000));
    objetivo.adicionar(ObjetivoIntervalo::criar(-0.99, 0.01));

    // Executando o algoritmo genetico
    let genetico = Genetico::criar(pop, evolucao, objetivo);
    let solucao = genetico.buscar_solucao();
    
    println!("{:?}", tempos);
    println!("{:?}", solucao); 
    
}

fn preencher_com_posicoes_aleatorias(pop: &mut Populacao<Vec<usize>>, qtd_maquinas: usize, qtd_amostras: usize, qtd_tarefas: usize) {

	let intervalo_maquinas = Range::new(0, qtd_maquinas);
    let mut aleatorio = thread_rng();
	for _ in 0..qtd_amostras {
		let mut amostra = Vec::with_capacity(qtd_tarefas);
		for _ in 0..qtd_tarefas {
			amostra.push(intervalo_maquinas.ind_sample(&mut aleatorio));
		}
		pop.adicionar(amostra);
	}
	
}

fn criar_tempos_aleatorios(qtd_tarefas: usize, tempo_maximo: usize) -> Vec<usize> {
	
	let intervalo_tempo = Range::new(0, tempo_maximo);
    let mut aleatorio = thread_rng();
    
    let mut tempos = Vec::with_capacity(qtd_tarefas);
    for _ in 0..qtd_tarefas {
    	tempos.push(intervalo_tempo.ind_sample(&mut aleatorio));
    }
    
    tempos
}

pub struct AptidaoTarefasMaquinas {
	qtd_maquinas: usize,
	tempo_task: Vec<usize>
}

impl Aptidao<Vec<usize>> for AptidaoTarefasMaquinas {
	
	fn calcular_aptidao(&self, genes: &Vec<usize>) -> f64 {
		
		let mut tasks_maquinas = Vec::new();
		tasks_maquinas.resize(self.qtd_maquinas, 0);
		// Calculando o tempo de tarefa para cada máquina
		for (maquina, tempo) in genes.iter().zip(self.tempo_task.iter()) {
			tasks_maquinas[*maquina] = tasks_maquinas[*maquina] + tempo;
		}
		let maior_tempo = tasks_maquinas.iter().max().unwrap();
		
		-(*maior_tempo as f64)
	}
	
} 

#[cfg(test)]
mod test {

    use super::*;
    use genetics::aptidao::Aptidao;

    #[test]
    fn deve_calcular_aptidao() {
    	let aptidao = AptidaoTarefasMaquinas{
    		qtd_maquinas:  5,
    		tempo_task: vec![1,1,1,1,1]
    	};
    	let tempo = aptidao.calcular_aptidao(&vec![0,1,2,3,4]);
    	assert_eq!(tempo as isize, -1);
    	
    	let tempo = aptidao.calcular_aptidao(&vec![0,0,0,0,0]);
    	assert_eq!(tempo as isize, -5);
    	
    	let tempo = aptidao.calcular_aptidao(&vec![0,0,0,1,1]);
    	assert_eq!(tempo as isize, -3);
    	
    }
}
