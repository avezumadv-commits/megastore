#[derive(Debug, Clone)]
pub struct Produto {
    pub id: usize,
    pub nome: String,
    pub categoria: String,
    pub preco: f64,
}

#[derive(Debug, Clone)]
pub struct Cliente {
    pub id: usize,
    pub nome: String,
}