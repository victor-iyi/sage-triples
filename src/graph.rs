use crate::triples::Triple;

use finalfusion::{prelude::*, storage::Storage, vocab::Vocab};
use ndarray::Array2;
use sprs::CsMat;

/// A Graph is a collection of triples, and an abstraction
/// for subgraphs in the Knowledge Graph.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Graph {
  triples: Vec<Triple>,
  nodes: Vec<String>,
  edges: Vec<String>,
  directed: bool,
}

impl Graph {
  /// Create a new empty graph.
  pub fn new() -> Graph {
    Graph {
      triples: Vec::new(),
      nodes: Vec::new(),
      edges: Vec::new(),
      directed: false,
    }
  }

  /// Add a triple to the graph.
  pub fn add_triple(&mut self, triple: Triple) {
    self.add_node(triple.subject().to_string());
    self.add_node(triple.object().to_string());
    self.add_edge(triple.relation().to_string());
    self.triples.push(triple);
  }

  /// Adjacency matrix representing the connection a node has with other nodes.
  /// It's a matrix of size `(n_nodes, n_nodes)`.
  pub fn adj_matrix(&self) -> Array2<u8> {
    let n = self.n_nodes();
    let mut matrix = Array2::zeros((n, n));
    for triple in &self.triples {
      let s = self.get_node_idx(triple.subject()).unwrap();
      let o = self.get_node_idx(triple.object()).unwrap();
      matrix[[s, o]] = 1;
    }
    matrix
  }
}

impl Graph {
  /// Node features is a 2-D array of size `(n_nodes, embedding_dims)`.
  pub fn node_features<V, S>(
    &self,
    embeddings: &Embeddings<V, S>,
  ) -> Array2<f32>
  where
    V: Vocab,
    S: Storage,
  {
    let mut matrix = Array2::zeros((self.n_nodes(), embeddings.dims()));
    for (i, node) in self.nodes.iter().enumerate() {
      if let Some(embedding) = embeddings.embedding(node) {
        matrix.row_mut(i).assign(&embedding);
      }
    }
    matrix
  }

  /// Edge embeddings is a 2-D array of size `(n_edges, embedding_dims)`.
  pub fn edge_embeddings<V, S>(
    &self,
    embeddings: &Embeddings<V, S>,
  ) -> Array2<f32>
  where
    V: Vocab,
    S: Storage,
  {
    let mut matrix = Array2::zeros((self.n_edges(), embeddings.dims()));
    for (i, edge) in self.edges.iter().enumerate() {
      if let Some(embedding) = embeddings.embedding(edge) {
        matrix.row_mut(i).assign(&embedding);
      }
    }
    matrix
  }

  /// Edge features is a sparse 2-D matrix of size `(n_edges, n_edges)`.
  pub fn edge_features(
    &self,
    // embeddings: &Embeddings<V, S>,
  ) -> CsMat<i32> {
    let n = self.n_edges();
    let mut matrix = Array2::zeros((n, n));
    for triple in &self.triples {
      let s = self.get_node_idx(triple.subject()).unwrap();
      let o = self.get_node_idx(triple.object()).unwrap();
      let r = self.get_edge_idx(triple.relation()).unwrap();
      matrix[[s, o]] = r as i32;
    }

    let sparse = CsMat::csr_from_dense(matrix.view(), -1);
    sparse
  }
}

impl Graph {
  /// Add unique nodes.
  fn add_node(&mut self, node: String) {
    if self.nodes.contains(&node) {
      return;
    }
    self.nodes.push(node);
  }

  /// Add unique edges if the graph is directed.
  fn add_edge(&mut self, edge: String) {
    if self.directed {
      if !self.edges.contains(&edge) {
        self.edges.push(edge);
      }
    } else {
      self.edges.push(edge);
    }
  }
}

impl Graph {
  /// Return the index of a node in the nodes vector.
  pub fn get_node_idx(&self, node: &str) -> Option<usize> {
    self.nodes.iter().position(|n| n == node)
  }

  /// Return the index of an edge in the edges vector.
  pub fn get_edge_idx(&self, edge: &str) -> Option<usize> {
    self.edges.iter().position(|e| e == edge)
  }

  /// Checks whether the graph is a directed graph.
  pub fn is_directed(&self) -> bool {
    self.directed
  }

  /// Check whether the graph is an undirected graph.
  pub fn is_undirected(&self) -> bool {
    !self.directed
  }

  /// Returns the number of triples in the graph.
  pub fn len(&self) -> usize {
    self.triples.len()
  }

  /// Checks if the graph is empty.
  pub fn is_empty(&self) -> bool {
    self.triples.is_empty()
  }

  /// Returns the number of nodes in the graph.
  pub fn n_nodes(&self) -> usize {
    self.nodes.len()
  }

  /// Returns the number of edges in the graph.
  pub fn n_edges(&self) -> usize {
    self.edges.len()
  }

  /// Returns the number of triples in the graph.
  pub fn n_triples(&self) -> usize {
    self.triples.len()
  }

  /// Returns the triples in the graph.
  pub fn triples(&self) -> &[Triple] {
    &self.triples
  }

  /// Returns the nodes in the graph.
  pub fn nodes(&self) -> &[String] {
    &self.nodes
  }

  /// Returns the edges in the graph.
  pub fn edges(&self) -> &[String] {
    &self.edges
  }
}

impl std::fmt::Debug for Graph {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut triples = String::new();
    for triple in &self.triples {
      triples.push_str(&format!("{:?}\n", triple));
    }
    write!(f, "{}", triples)
  }
}

impl std::fmt::Display for Graph {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut triples = String::new();
    for triple in &self.triples {
      triples.push_str(&format!("{}\n", triple));
    }
    write!(f, "{}", triples)
  }
}

impl Default for Graph {
  fn default() -> Graph {
    Graph::new()
  }
}

impl From<Vec<Triple>> for Graph {
  fn from(triples: Vec<Triple>) -> Graph {
    let mut graph = Graph::new();
    for triple in triples {
      graph.add_triple(triple);
    }
    graph
  }
}

impl From<&[(&str, &str, &str)]> for Graph {
  fn from(triples: &[(&str, &str, &str)]) -> Graph {
    let mut graph = Graph::new();
    for triple in triples {
      graph.add_triple(Triple::new(
        triple.0.to_string(),
        triple.1.to_string(),
        triple.2.to_string(),
      ));
    }
    graph
  }
}
