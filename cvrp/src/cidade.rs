
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub struct Problema {
    pub qtd_caminhoes: usize,
    pub indice_deposito: usize,
    pub cidades: Vec<Cidade>,
}

#[derive(Debug)]
pub struct Cidade {
    pub numero: usize,
    pub x: i32,
    pub y: i32,
    pub demanda: i32,
}

enum AreaArquivo {
    InicioLeitura,
    PosicaoCidade,
    DemandaCidade,
    PosicaoCidadeDeposito,
}



pub fn ler(caminho_arquivo: &str) -> Problema {

    let caminho = Path::new(caminho_arquivo);
    let arquivo = match File::open(caminho) {
        Ok(arquivo) => arquivo,
        Err(why) => panic!("error {}", why),
    };

    let leitor = BufReader::new(arquivo);

    let mut indice_deposito = 0;
    let mut cidades = Vec::new();
    let mut trecho = AreaArquivo::InicioLeitura;
    for linha in leitor.lines() {

        let conteudo: String = match linha {
            Ok(conteudo) => conteudo,
            Err(why) => panic!("Erro ao processar linha, causa: {}", why),
        };

        let conteudo_limpo = conteudo.trim();

        if "NODE_COORD_SECTION" == conteudo_limpo {
            trecho = AreaArquivo::PosicaoCidade;
            continue;
        } else if "DEMAND_SECTION" == conteudo_limpo {
            trecho = AreaArquivo::DemandaCidade;
            continue;
        } else if "DEPOT_SECTION" == conteudo_limpo {
            trecho = AreaArquivo::PosicaoCidadeDeposito;
            continue;
        } else if "EOF" == conteudo_limpo {
            break;
        }

        match trecho {
            AreaArquivo::InicioLeitura => {}
            AreaArquivo::PosicaoCidade => {
                cidades.push(ler_cidade(&conteudo));
            }
            AreaArquivo::PosicaoCidadeDeposito => {
                if "-1" == conteudo_limpo {
                    continue;
                }
                indice_deposito = FromStr::from_str(conteudo_limpo)
                                      .expect("Erro ao processar indice do deposito");
            }
            AreaArquivo::DemandaCidade => {
                atualiza_demanda(&mut cidades, &conteudo);
            }
        }
    }

    return Problema {
        indice_deposito: indice_deposito - 1,
        cidades: cidades,
        qtd_caminhoes: 5,
    };
}

use std::str::FromStr;

fn atualiza_demanda(mut cidades: &mut Vec<Cidade>, linha: &String) {
    let mut partes = linha.split_whitespace();

    let indice_str = partes.next().expect("Erro ao ler valor x");
    let demand_str = partes.next().expect("Erro ao ler valor y");

    let indice: usize = FromStr::from_str(indice_str).expect("Erro ao ler indice");
    let demanda: i32 = FromStr::from_str(demand_str).expect("Erro ao ler demanda");

    cidades[indice - 1].demanda += demanda;
}

fn ler_cidade(linha: &String) -> Cidade {
    let mut partes = linha.split_whitespace();

    let numero_str = partes.next().expect("Erro ao ler numedo da cidade");
    let x_str = partes.next().expect("Erro ao ler X");
    let y_str = partes.next().expect("Erro ao ler Y");

    Cidade {
        numero: FromStr::from_str(numero_str).expect("1"),
        x: FromStr::from_str(x_str).expect("2"),
        y: FromStr::from_str(y_str).expect("3"),
        demanda: 0,
    }
}

#[test]
fn deve_atualizar_demanda() {
    let mut cidades = vec![Cidade {
                               numero: 1,
                               x: 0,
                               y: 0,
                               demanda: 1,
                           },
                           Cidade {
                               numero: 3,
                               x: 0,
                               y: 0,
                               demanda: 2,
                           },
                           Cidade {
                               numero: 2,
                               x: 0,
                               y: 0,
                               demanda: 3,
                           }];
    atualiza_demanda(&mut cidades, &"2 30".into());
    assert_eq!(cidades[0].demanda, 1);
    assert_eq!(cidades[1].demanda, 32);
    assert_eq!(cidades[2].demanda, 3);
}

#[test]
fn deve_ler_cidade() {
    let cidade = ler_cidade(&"1 22 44".to_string());
    assert_eq!(cidade.numero, 1);
    assert_eq!(cidade.x, 22);
    assert_eq!(cidade.y, 44);
}
