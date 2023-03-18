use ndarray::Array2;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Triple(String, String, String);

impl Triple {
  /// Create a new s-r-o triple.
  pub fn new(s: String, r: String, o: String) -> Triple {
    Triple(s, r, o)
  }

  /// Get the subject of the triple.
  pub fn subject(&self) -> &str {
    &self.0
  }

  /// Get the relation (or predicate) of the triple.
  pub fn relation(&self) -> &str {
    &self.1
  }

  /// Get the object of the triple.
  pub fn object(&self) -> &str {
    &self.2
  }
}

impl std::fmt::Debug for Triple {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Triple({} -- {} -- {})", self.0, self.1, self.2)
  }
}

impl std::fmt::Display for Triple {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({} -- {} -- {})", self.0, self.1, self.2)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Graph {
  triples: Vec<Triple>,
  nodes: Vec<String>,
  edges: Vec<String>,
  directed: bool,
}

impl Graph {
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
  pub fn get_node_idx(&self, node: &str) -> Option<usize> {
    self.nodes.iter().position(|n| n == node)
  }

  pub fn is_directed(&self) -> bool {
    self.directed
  }

  pub fn is_undirected(&self) -> bool {
    !self.directed
  }

  pub fn len(&self) -> usize {
    self.triples.len()
  }

  pub fn is_empty(&self) -> bool {
    self.triples.is_empty()
  }

  pub fn n_nodes(&self) -> usize {
    self.nodes.len()
  }

  pub fn n_edges(&self) -> usize {
    self.edges.len()
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

impl From<Vec<(&str, &str, &str)>> for Graph {
  fn from(triples: Vec<(&str, &str, &str)>) -> Graph {
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
