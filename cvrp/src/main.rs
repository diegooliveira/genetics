
extern crate genetics;
extern crate simplelog;
#[macro_use]
extern crate log;


mod cidade;
use cidade::{Problema, Cidade};

use genetics::aptidao::Aptidao;
use genetics::aleatorio::Aleatorio;
use genetics::evolucao::cruzamento::*;
use genetics::objetivo::NumeroMaximoIteracoes;
use genetics::evolucao::selecao::SelecaoPorTorneio;
use genetics::evolucao::EvolucaoMista;
use genetics::observador::ObservadorEvolucaoImprimeAptidao;
use genetics::genetico::Genetico;
use genetics::populacao::{Populacao, CriadorIndividuos, Operacao};

use std::rc::Rc;
use std::collections::HashMap;

use simplelog::{TermLogger, CombinedLogger, LogLevelFilter};

#[derive(Clone,Debug)]
struct Rota {
	alterada: bool,
	cidades: Vec<usize>,
}

fn main() {

//	CombinedLogger::init(vec![TermLogger::new(LogLevelFilter::Trace)]).unwrap();

    debug!("Iniciando");
    let problema = cidade::ler("/home/diego/projects/genetics/cvrp/dados/problemas/A-n32-k5.vrp");
    let problema = Rc::new(problema);

    let aptidao = AptidaoRota { problema: problema.clone() };
    let criador_rotas = CriadorRotas { problema: problema.clone() };
    let cruzador_rotas = CruzadorRotas { problema: problema.clone() };

    let populacao = Populacao::criar(aptidao, &criador_rotas, 100, Operacao::Min);
    let mut evolucao = EvolucaoMista::criar();
    evolucao.adicionar(Cruzamento::criar(SelecaoPorTorneio::criar(5), cruzador_rotas, 0.9));

    let objetivo = NumeroMaximoIteracoes::criar(1000);

    let mut genetico = Genetico::criar(populacao, evolucao, objetivo);
    genetico.observador(ObservadorEvolucaoImprimeAptidao);

    let solucao = genetico.buscar_solucao();
    println!("Solucao:");
    for rota in solucao {
	    println!("\t{:?}", rota.cidades);	
    }
}

struct CriadorRotas {
    problema: Rc<Problema>,
}

struct AptidaoRota {
    problema: Rc<Problema>,
}

struct CruzadorRotas {
    problema: Rc<Problema>,
}

impl Cruzador<Vec<Rota>> for CruzadorRotas {
    fn cruzar(&self,
              aleatorio: &mut Aleatorio,
              primeiro: &Vec<Rota>,
              segundo: &Vec<Rota>)
              -> (Vec<Rota>, Vec<Rota>) {

        

        // sorteando qual rota vamos cruzar
        let indice_caminhao_cruzar = aleatorio.intervalo(0, primeiro.len());
		
        // Escolhendo o ponto de cruzamento da rota escolhida
        let primeiro_caminhao = &primeiro[indice_caminhao_cruzar];
        let segundo_caminhao = &segundo[indice_caminhao_cruzar];

        let menor_rota = std::cmp::min(primeiro_caminhao.cidades.len(), segundo_caminhao.cidades.len());
        let maior_rota = std::cmp::max(primeiro_caminhao.cidades.len(), segundo_caminhao.cidades.len());
        
        if menor_rota == 0{
        
	        return (primeiro. clone(), segundo.clone());	
        }
        let indice_cruzar = aleatorio.intervalo(0, menor_rota);
		
		
		debug!("Cruzando rotas na posicao Rota: {}, Cidade: {}", indice_caminhao_cruzar, indice_cruzar);
        debug!("  R1: {:?}", primeiro);
        debug!("  R2: {:?}", segundo);

        // Criando os filhos
        let mut primeiro_filho = primeiro.clone();
        let mut segundo_filho = segundo.clone();

        let mut primeira_rota_filha = Vec::new();
        let mut segunda_rota_filha = Vec::new();
        for i in 0..maior_rota {

            if i < indice_cruzar {

                if i < primeiro_caminhao.cidades.len() {
                    primeira_rota_filha.push(primeiro_caminhao.cidades[i]);
                }
                if i < segundo_caminhao.cidades.len() {
                    segunda_rota_filha.push(segundo_caminhao.cidades[i]);
                }
            } else {

                if i < segundo_caminhao.cidades.len() {
                    primeira_rota_filha.push(segundo_caminhao.cidades[i]);
                }
                if i < primeiro_caminhao.cidades.len() {
                    segunda_rota_filha.push(primeiro_caminhao.cidades[i]);
                }
            }
        }

        // Colocando a rota mesclada no conjunto de rotas, é preciso remover para
        // adicionar por que copiamos os valores para as demais colunas.
        primeiro_filho.remove(indice_caminhao_cruzar);
        segundo_filho.remove(indice_caminhao_cruzar);

        primeiro_filho.insert(indice_caminhao_cruzar, Rota {
        		alterada: true,
        		cidades: primeira_rota_filha,
        });
        segundo_filho.insert(indice_caminhao_cruzar, Rota {
        		alterada: true,
        		cidades: segunda_rota_filha
        });
        
        debug!("  F1_antes: {:?}", primeiro_filho);
        debug!("  F2_antes: {:?}", segundo_filho);

        // Corrigindo qualquer problema que possa ter acontecido quando
        // cruzamos
        corrige_rota_duplicada(&mut primeiro_filho, &mut segundo_filho, aleatorio);
        corrige_rota_duplicada(&mut segundo_filho, &mut primeiro_filho, aleatorio);
        
        
	    
        //aborta_se_invalido(&primeiro_filho);

//        otimiza_local_rotas(&mut primeiro_filho,
//                            &self.problema.cidades,
//                            self.problema.indice_deposito);
//        otimiza_local_rotas(&mut segundo_filho,
//                            &self.problema.cidades,
//                            self.problema.indice_deposito);
//
//        debug!("  F1_depois: {:?}", primeiro_filho);
//        debug!("  F2_depois: {:?}", segundo_filho);

        (primeiro_filho, segundo_filho)
    }
}

fn aborta_se_invalido(rotas_calc: &Vec<Rota>) {

  let mut contagem = HashMap::new();
        for (idx_rota, rota) in rotas_calc.iter().enumerate() {
	        for (idx_cidade, cidade) in rota.cidades.iter().enumerate() {
	            let mut indices = contagem.entry(*cidade).or_insert_with(|| Vec::new());
	            indices.push((idx_rota, idx_cidade));
	        }
	    }
        
        for(cidade, rotas) in &contagem {
        	if rotas.len() > 1{
        		panic!(
"Erro 
    Cidade:{}
    Rotas: {:?}
      F1: {:?}", cidade, rotas, rotas_calc);
        	}
        }
        
        let tot = rotas_calc.iter().fold(0, |acc, val| {
        		acc + val.cidades.iter().fold(0, |acc2, val2| acc2 + val2)
        });
        if tot != 496 {
        	panic!("Faltando elementos (total:496 - {} = {} ) {:?}", tot, 496 - tot, rotas_calc);
        }
        
}

/// Faz z otimização local de cada rota, de tal forma que a sequencia das cidades seja a menor
/// possível
fn otimiza_local_rotas(rotas: &mut Vec<Rota>,
                       cidades: &Vec<Cidade>,
                       indice_deposito: usize) {

	
	let mut cache_distancias = HashMap::new();
    for mut rota in rotas.iter_mut() {
    	
    	if rota.cidades.len() == 0 || rota.alterada == false {
			continue;
    	}
    	
    	rota.alterada == false;

        let mut sequencia_atual = Vec::new();
        let mut melhor_sequencia = Vec::new();
        otimiza_local_rota(&mut rota.cidades, &mut sequencia_atual, &mut melhor_sequencia, 0.0, &cidades, indice_deposito, &mut cache_distancias);

		rota.cidades.clear();
		rota.cidades.extend_from_slice(&melhor_sequencia);
    }
    
}

fn otimiza_local_rota(mut rota: &mut Vec<usize>, // A rota que estamos trabalhando
                      mut sequencia_atual: &mut Vec<usize>, // O caminho atual
                      mut melhor_sequencia: &mut Vec<usize>, // O melhor caminho encontrado
                      mut melhor_distancia:  f64,
                      cidades: &Vec<Cidade>, // A cidade para calcularmos a distancia da rota atual
                      indice_deposito: usize, /* Referencia para o deposito */
                      mut cache_distancias: &mut HashMap<usize,f64>
						)
                      -> f64 {

	
	trace!("Agora: {:?}", sequencia_atual);

    // Caso a rota esteja vazia é por que está na hora dee calcular a distancia
    // dessa configuração
    if rota.is_empty() {

		let mut calcula_distancia = |idx_origen, idx_destino| {
			let de: &Cidade = &cidades[idx_origen];
	        let para: &Cidade = &cidades[idx_destino];
	        
	        let idx = de.numero << 32 | para.numero;
	        let dist = cache_distancias.entry(idx).or_insert_with(|| distancia(de, para) );
	        
	        trace!("\t({},{}) -> ({}, {}) = {}", de.x, de.y, para.x, para.y, dist);
	        
			*dist
		};
	
		// Do deposito até a primeira cidade
		let mut distancia_rota = calcula_distancia(indice_deposito, sequencia_atual[0]);
		
		// Rotas entre as cidades 
        for indice_atual in 1..sequencia_atual.len() {
            distancia_rota += calcula_distancia(
            	sequencia_atual[indice_atual - 1], 
            	sequencia_atual[indice_atual]);
        }
        // Da última cidade da rota até o deposito
        let ultima_cidade = sequencia_atual.len() -1;
        distancia_rota += calcula_distancia(
            	sequencia_atual[ultima_cidade], 
            	sequencia_atual[indice_deposito]);
        
		
        if melhor_sequencia.is_empty() {
            melhor_sequencia.extend_from_slice(&sequencia_atual);
        } else if distancia_rota < melhor_distancia {
            melhor_sequencia.clear();
            melhor_sequencia.extend_from_slice(&sequencia_atual);
        }
		trace!("\t tota: {} ", distancia_rota);
        return distancia_rota;
    }

    // Combinando a rota de todas as forma possiveis
    for i in 0..rota.len() {
        let indice_cidade = rota.remove(i);
        sequencia_atual.push(indice_cidade);
        

        let distancia = otimiza_local_rota(&mut rota,
                                           &mut sequencia_atual,
                                           &mut melhor_sequencia,
                                           melhor_distancia,
                                           &cidades,
                                           indice_deposito, 
									        &mut cache_distancias);
        
        if melhor_distancia == 0.0 || distancia < melhor_distancia {
        	melhor_distancia = distancia;
        } 
        
        sequencia_atual.pop();
        rota.insert(i, indice_cidade);
    }
    
    return melhor_distancia;

}

#[test]
fn teste_otimiza_local_rota() {
	
	let cidades = vec![
		Cidade { numero: 0, x: 0, y: 0, demanda: 0},
		Cidade { numero: 0, x: 1, y: 0, demanda: 0},
		Cidade { numero: 0, x: 2, y: 0, demanda: 0},
		Cidade { numero: 0, x: 3, y: 0, demanda: 0}
	];
	let mut rota = vec![3,2,1,0];
	let mut sequencia_atual = Vec::new();
	let mut melhor_sequencia = Vec::new();
	let mut cache_distancias = HashMap::new();
	otimiza_local_rota(&mut rota, &mut sequencia_atual, &mut melhor_sequencia, 0.0, &cidades, 0, &mut cache_distancias);
	
	
	println!("===> {:?}", melhor_sequencia);
	panic!("falhou");
	
}


fn corrige_rota_duplicada(primeiro: &mut Vec<Rota>,
                          segundo: &mut Vec<Rota>,
                          alet: &mut Aleatorio)
                          -> bool {


    debug!("Corrigindo rota duplicada");

    let mut counts_cidades = HashMap::new();

    // Contando em quantas rotas cada cidade aparece
    // o resultado desse processamento irá corrigir quando acontecer alguma coisa no formato
    //
    //  Pai 1)           Pai 2)
    //  r0 [ 1, 3, 9 ]   [ 2, 4, 6 ]
    //  r1 [ 2, 4, 6 ]   [ 8, 5, 7 ]
    //  r2 [ 5, 7, 8 ]   [ 1, 3, 9 ]
    //
    //        O cruzamento aconteceu na rota r2, cidade no indice 1, logo temos como filhos
    //
    //  Filho 1)         Filho 2)
    //  r1 [ 1, 3, 9 ]   [ 2, 4, 6 ]
    //  r2 [ 2, 4, 6 ]   [ 8, 5, 7 ]
    //  r3 [ 5, 7, 9 ]   [ 1, 3, 8 ]
    //             ^             ^
    // 	           Acontecem mais de uma vez por solução.
    //
    for (idx_caminhao, rota) in primeiro.iter().enumerate() {
        for (idx_cidade, cidade) in rota.cidades.iter().enumerate() {
            let mut indices = counts_cidades.entry(*cidade).or_insert_with(|| Vec::new());
            indices.push((idx_caminhao, idx_cidade));
        }
    }

    // Organizando por rota
    let mut correcao_rota = HashMap::new();
    for (_, rotas) in &counts_cidades {

        if rotas.len() > 1 {

            let (idx_rota, idx_cidade) = if alet.chance() > 0.5 {
                rotas[0]
            } else {
                rotas[1]
            };
            let movimentacoes = correcao_rota.entry(idx_rota).or_insert_with(|| Vec::new());
            movimentacoes.push(idx_cidade);
            movimentacoes.sort();
        }
    }

	debug!("Corrigindo {:?}", correcao_rota);

    for (idx_rota, cidades) in correcao_rota.iter_mut() {

        for idx_cidade in (0..cidades.len()).rev() {
        	let vai_para = alet.intervalo(0, segundo.len());
			debug!("Removendo da rota {}, cidade idx {} para rota {}", idx_rota, cidades[idx_cidade], vai_para);
            // Referencia para a cidade duplicada
            primeiro[*idx_rota].alterada = true;
            segundo[vai_para].alterada = true;
            
            let cidade = primeiro[*idx_rota].cidades.remove(cidades[idx_cidade]);
            
            let tam = segundo[vai_para].cidades.len();
            if tam == 0 {
	            segundo[vai_para].cidades.push(cidade);	
            } else{
	            let posicao = alet.intervalo(0, tam);
	            segundo[vai_para].cidades.insert(posicao, cidade);
            } 
        }
    }

    correcao_rota.len() > 0
}





impl CriadorIndividuos<Vec<Rota>> for CriadorRotas {
    fn criar(&self, aleatorio: &mut Aleatorio) -> Vec<Rota> {

        debug!("Criando individuo aleatorio");

        let cidades = &self.problema.cidades;
        let qtd_qtd_caminhoes = self.problema.qtd_caminhoes;

        let mut rotas = Vec::new();

        let mut indices: Vec<usize> = (0..cidades.len()).collect();

        let mut qtd_cidades_caminhao = cidades.len() / qtd_qtd_caminhoes;
        if qtd_cidades_caminhao * qtd_qtd_caminhoes < cidades.len() {
            qtd_cidades_caminhao += 1;
        }

        debug!("Cidades: {} - Rotas: {} - Cidades por rota: {}",
               cidades.len(),
               qtd_qtd_caminhoes,
               qtd_cidades_caminhao);
        for _ in 0..qtd_qtd_caminhoes {

            debug!("Cidades disponiveis: {}", indices.len());
            let mut cidades = Vec::new();

            for _ in 0..qtd_cidades_caminhao {

                let qtd = indices.len();

                if qtd == 0 {
                    break;
                }

                let idx = aleatorio.intervalo(0, qtd);


                let cidade = indices.remove(idx);
                cidades.push(cidade);
            }
            rotas.push(Rota{ alterada: true, cidades: cidades});
        }
        debug!("Rota aleatoria: {:?}", rotas);
        
        
//        otimiza_local_rotas(&mut rotas,
//                            &self.problema.cidades,
//                            self.problema.indice_deposito);
        
        //aborta_se_invalido(&rotas);
        rotas
    }
}


impl Aptidao<Vec<Rota>> for AptidaoRota {
    fn calcular_aptidao(&self, genes: &Vec<Rota>) -> f64 {
        use std::ops::Index;

        debug!("Calculando aptidao. rota={:?}", genes);

        let cidades = &self.problema.cidades;
        let indice_deposito = self.problema.indice_deposito;

        let mut dist_total = 0.0;
        for rota in genes {

			if rota.cidades.len() ==0 {
				continue;
			}

            let mut distancia_rota = 0.0;
            for (posicao, idx_cidade) in rota.cidades.iter().enumerate() {

                let de = if posicao == 0 {
                    cidades.index(indice_deposito)
                } else {
                    let idx = posicao - 1;
                    cidades.index(idx)
                };

                let para = cidades.index(*idx_cidade);
                let distancia_cidades = distancia(de, para);

                distancia_rota += distancia_cidades;
            }

            // Voltando para o deposito
            distancia_rota += distancia(cidades.index(rota.cidades[rota.cidades.len() - 1]),
                                        cidades.index(indice_deposito));

            dist_total += distancia_rota;
        }

        dist_total
    }
}

fn distancia(de: &Cidade, para: &Cidade) -> f64 {
    use std::f64;

    let dist = (((de.x - para.x) as f64).powi(2) + ((de.y - para.y) as f64).powi(2)).sqrt();
    dist
}

#[test]
fn calculo_distancia() {

    let de = Cidade {
        numero: 1,
        x: 15,
        y: 17,
        demanda: 0,
    };
    let para = Cidade {
        numero: 2,
        x: 14,
        y: 9,
        demanda: 0,
    };

    let dist = distancia(&de, &para);
    assert_eq!(dist, 8.06225774829855);

}

#[test]
fn deve_criar_cidades() {


    // Criando uma cidade com 5 caminhoes e 10 rotas
    let problema = Problema {
        indice_deposito: 0,
        qtd_caminhoes: 5,
        cidades: vec![
			Cidade { numero: 0, x: 15, y: 17, demanda: 0},
			Cidade { numero: 1, x: 14, y:  9, demanda: 0},
			Cidade { numero: 2, x: 11, y: 13, demanda: 0},
			Cidade { numero: 3, x: 16, y: 18, demanda: 0},
			Cidade { numero: 4, x:  1, y: 12, demanda: 0},
			Cidade { numero: 5, x: 13, y: 19, demanda: 0},
			Cidade { numero: 6, x:  4, y: 20, demanda: 0},
			Cidade { numero: 7, x: 17, y:  2, demanda: 0},
			Cidade { numero: 8, x: 12, y:  6, demanda: 0},
			Cidade { numero: 9, x: 14, y:  8, demanda: 0},
		],
    };

    // Criando uma rota aleatoria
    let mut aleatorio = Aleatorio::criar();
    let criador_rotas = CriadorRotas { problema: Rc::new(problema) };
    let possivel_rota = criador_rotas.criar(&mut aleatorio);

    // Imprimindo a rota para debug
    println!("Rota criada");
    for rota_caminhao in possivel_rota.iter() {
        for idx_cidade in rota_caminhao.cidades.iter() {
            print!("\t{}", idx_cidade);
        }
        println!("");
    }

    // Validando
    assert_eq!(possivel_rota.len(), 5); // deve ter uma rota para cada caminhão
    for rota_caminhao in possivel_rota {
        assert_eq!(rota_caminhao.cidades.len(), 2); // cada caminhão deve tres cidades
    }
}

#[test]
/// Testa se o calculo da aptidão para o problema
fn deve_calcular_aptidao() {

    let rotas = vec![
	    Rota{
	    	alterada: true,
	    	cidades: vec![0, 1, 2]
	    },
	     Rota{
	    	alterada: true,
	    	cidades: vec![3, 4, 5]},
	     Rota{
	    	alterada: true,
	    	cidades: vec![6, 7, 8]}
	     ];

    let problema = Problema {
        indice_deposito: 0,
        qtd_caminhoes: 3,
        cidades: vec![
			Cidade { numero: 1, x: 15, y: 17, demanda: 0},
			Cidade { numero: 1, x: 14, y:  9, demanda: 0},
			Cidade { numero: 1, x: 11, y: 13, demanda: 0},
			Cidade { numero: 1, x: 16, y: 18, demanda: 0},
			Cidade { numero: 1, x:  1, y: 12, demanda: 0},
			Cidade { numero: 1, x: 13, y: 19, demanda: 0},
			Cidade { numero: 1, x:  4, y: 20, demanda: 0},
			Cidade { numero: 1, x: 17, y:  2, demanda: 0},
			Cidade { numero: 1, x: 12, y:  6, demanda: 0},
		],
    };

    let aptidao_rota = AptidaoRota { problema: Rc::new(problema) };
    let aptidao = aptidao_rota.calcular_aptidao(&rotas);
    assert_eq!(aptidao, 104.41992714635366);


}
