use std::collections::HashMap;

#[derive(Debug)]
pub struct GrafoMegaStore {
    // Mapeia o ID do produto para os IDs dos produtos vizinhos/relacionados
    pub adj_list: HashMap<usize, Vec<usize>>,
    // Mapeia o ID do cliente para os produtos que ele comprou
    pub cliente_compras: HashMap<usize, Vec<usize>>,
}

impl GrafoMegaStore {
    pub fn new() -> Self {
        GrafoMegaStore {
            adj_list: HashMap::new(),
            cliente_compras: HashMap::new(),
        }
    }

    // Adiciona um produto como vértice no grafo
    pub fn adicionar_produto(&mut self, produto_id: usize) {
        self.adj_list.entry(produto_id).or_insert_with(Vec::new);
    }

    // Cria uma conexão (aresta) bidirecional entre dois produtos (ex: "quem comprou X também comprou Y")
    pub fn adicionar_conexao_produtos(&mut self, p1: usize, p2: usize) {
        self.adj_list.entry(p1).or_insert_with(Vec::new).push(p2);
        self.adj_list.entry(p2).or_insert_with(Vec::new).push(p1);
    }

    // Registra que um cliente comprou determinado produto
    pub fn registrar_compra_cliente(&mut self, cliente_id: usize, produto_id: usize) {
        self.cliente_compras.entry(cliente_id).or_insert_with(Vec::new).push(produto_id);
    }
}