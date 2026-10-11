mod models;
mod graph;
mod recommendation;

use models::{Produto, Cliente};
use graph::GrafoMegaStore;
use recommendation::recomendar_para_cliente;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("=== PROJETO CONECTASTORE: SISTEMA DE RECOMENDAÇÃO BASEADO EM GRAFOS ===");

    // 1. Inicializa o Grafo e o catálogo de produtos com HashMap
    let mut grafo = GrafoMegaStore::new();
    let mut catalogo_produtos: HashMap<usize, Produto> = HashMap::new();

    // 2. Cadastra produtos fictícios na MegaStore
    let produtos_iniciais = vec![
        Produto { id: 1, nome: String::from("Smartphone Galaxy"), categoria: String::from("Eletrônicos"), preco: 2500.0 },
        Produto { id: 2, nome: String::from("Fone Bluetooth"), categoria: String::from("Eletrônicos"), preco: 300.0 },
        Produto { id: 3, nome: String::from("Camisa Polo"), categoria: String::from("Vestuário"), preco: 120.0 },
        Produto { id: 4, nome: String::from("Tênis Esportivo"), categoria: String::from("Vestuário"), preco: 350.0 },
        Produto { id: 5, nome: String::from("Livro de Rust Avançado"), categoria: String::from("Livros"), preco: 150.0 },
    ];

    for p in produtos_iniciais {
        grafo.adicionar_produto(p.id);
        catalogo_produtos.insert(p.id, p);
    }

    // 3. Conecta os produtos no Grafo (Relação de similaridade)
    grafo.adicionar_conexao_produtos(1, 2); // Smartphone <-> Fone
    grafo.adicionar_conexao_produtos(3, 4); // Camisa <-> Tênis
    grafo.adicionar_conexao_produtos(1, 5); // Smartphone <-> Livro de Rust

    // 4. Simula um cliente e suas compras anteriores
    let cliente = Cliente { id: 100, nome: String::from("Carlos Silva") };
    grafo.registrar_compra_cliente(cliente.id, 1); // Comprou o Smartphone (ID 1)

    println!("\nCliente: {} (ID: {})", cliente.nome, cliente.id);
    println!("Histórico de compras: Produto ID 1 (Smartphone Galaxy)");

    // 5. Medição de tempo de execução (Benchmark básico)
    let inicio = Instant::now();
    let ids_recomendados = recomendar_para_cliente(cliente.id, &grafo);
    let duracao = inicio.elapsed();

    // 6. Exibe os resultados na tela
    println!("\n--- PRODUTOS RECOMENDADOS PELO GRAFO ---");
    for id_prod in ids_recomendados {
        if let Some(prod) = catalogo_produtos.get(&id_prod) {
            println!("- Recomendado: {} [Categoria: {}] (R$ {:.2})", prod.nome, prod.categoria, prod.preco);
        }
    }

    println!("\n[Desempenho] Tempo de processamento: {:?}\n", duracao);
}