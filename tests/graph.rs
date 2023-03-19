use ndarray::array;
use sage_triples::Graph;

// Sample s-r-o triples.
static SRO: &[(&str, &str, &str)] = &[
  ("simon", "plays", "tennis"),
  ("simon", "lives", "melbourne"),
  ("tennis", "sport", "melbourne"),
  ("melbourne", "located", "australia"),
  ("tennis", "plays", "simon"),
  ("melbourne", "lives", "simon"),
  ("melbourne", "sport", "tennis"),
  ("australia", "located", "melbourne"),
];

#[test]
fn test_graph() {
  let graph = Graph::from(SRO);
  assert_eq!(graph.n_nodes(), 4);
  assert_eq!(graph.n_edges(), 8);
  assert_eq!(graph.n_triples(), 8);
  assert!(graph.is_undirected());
}

#[test]
fn test_adj_matrix() {
  let graph = Graph::from(SRO);
  assert_eq!(
    graph.adj_matrix(),
    array![[0, 1, 1, 0], [1, 0, 1, 0], [1, 1, 0, 1], [0, 0, 1, 0]]
  );
}

#[test]
fn test_edge_features() {
  let graph = Graph::from(SRO);
  let dense_matrix = array![
    [0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 2, 0, 0, 0, 0, 0],
    [1, 2, 0, 3, 0, 0, 0, 0],
    [0, 0, 3, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0]
  ];
  let edge_features = graph.edge_features();
  assert_eq!(edge_features.to_dense(), dense_matrix);
}
