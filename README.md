# ConectaStore - Sistema de Recomendação Baseado em Grafos (MegaStore)

Sistema de recomendação de produtos desenvolvido em **Rust** utilizando a **Teoria dos Grafos** e estruturas de dados de alta eficiência para a plataforma de e-commerce **MegaStore**.

---

## 📌 Sobre o Projeto

A MegaStore comercializa milhões de produtos e necessitava de uma solução avançada para personalizar as recomendações enviadas aos seus clientes. O **ConectaStore** substitui métodos tradicionais baseados em meros rankings de vendas por uma estrutura interconectada de dados, mapeando relações comportamentais, histórico de compras e similaridade entre produtos.

---

## 🛠️ Arquitetura e Estruturas de Dados

- **Grafo Direcionado e Ponderado:** Representa os produtos, clientes e categorias como vértices, conectando-os por arestas de histórico de compras e relações de similaridade.
- **Lista de Adjacência (`HashMap<usize, Vec<usize>>`):** Escolhida em detrimento da matriz de adjacência por ser altamente otimizada em termos de memória para grafos esparsos.
- **Tabela Hash (`HashMap`):** Garante acesso e busca rápida por identificadores únicos em tempo constante $O(1)$.
- **Algoritmo de Recomendação (BFS):** Utiliza um percurso inspirado em Busca em Largura para explorar conexões de produtos.
- **Prevenção de Duplicados (`HashSet`):** Assegura que itens já adquiridos ou visitados não sejam recomendados novamente.

---

## ⚡ Desempenho e Complexidade

- **Complexidade de Tempo:** $O(V + E)$ durante o percurso de recomendação.
- **Complexidade de Espaço:** $O(V + E)$, otimizando o uso da memória RAM.
- **Benchmark:** Processamento das recomendações em escassos microssegundos (ex: ~7.9 µs), aproveitando o gerenciamento de memória do Rust via *ownership* e *borrowing* sem a sobrecarga de um *garbage collector*.

---

## 💻 Como Executar o Projeto

### Pré-requisitos
- **Rust e Cargo** instalados na máquina (`rustc` e `cargo`).

### Passos
1. Clone o repositório:
   ```bash
   git clone [https://github.com/SEU_USUARIO_GITHUB/megastore.git](https://github.com/SEU_USUARIO_GITHUB/megastore.git)