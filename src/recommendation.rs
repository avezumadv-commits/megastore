use std::collections::{HashSet, VecDeque};
use crate::graph::GrafoMegaStore;

pub fn recomendar_para_cliente(
    cliente_id: usize,
    grafo: &GrafoMegaStore,
) -> Vec<usize> {
    let mut recomendacoes = Vec::new();
    let mut visitados = HashSet::new();
    let mut fila = VecDeque::new();

    // 1. Obtém os produtos que o cliente já comprou no histórico
    if let Some(produtos_comprados) = grafo.cliente_compras.get(&cliente_id) {
        for &produto_id in produtos_comprados {
            visitados.insert(produto_id);
            fila.push_back(produto_id);
        }
    } else {
        return recomendacoes; // Se o cliente não tiver histórico, retorna vazio
    }

    // 2. Percorre as conexões do grafo (BFS) para achar produtos relacionados
    while let Some(produto_atual) = fila.pop_front() {
        if let Some(vizinhos) = grafo.adj_list.get(&produto_atual) {
            for &vizinho in vizinhos {
                if !visitados.contains(&vizinho) {
                    visitados.insert(vizinho);
                    recomendacoes.push(vizinho);
                }
            }
        }
    }

    recomendacoes
}