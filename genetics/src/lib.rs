//! Biblioteca com rotinas que possibilitam o uso de algoritmos genéticos (AG).
extern crate rand;
#[macro_use]
extern crate log;
extern crate crossbeam;

/// Interface simplificada para o gerador de números aleatórios.
pub mod aleatorio;
/// Métodos e estruturas que permitem interagir com a população.
pub mod populacao;
/// Estruturas e métodos aplicados na evolução.
pub mod evolucao;
/// Implementação do AG.
pub mod genetico;
/// Estrutura que permite flexibilizar os objetivos do AG.
pub mod objetivo;
/// Definição da interface da função de calculo da aptidão dos indivíduos.
pub mod aptidao;
/// Definição o contrato para implementações que desejam observar a evolução do AG.
pub mod observador;
