use crate::triples::Triple;
use ndarray::Array2;

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
