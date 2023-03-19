use sage_triples::Graph;

// use std::fs::File;
// use std::io::BufReader;
// use finalfusion::prelude::*;

fn main() {
  let sro = Vec::from([
    ("simon", "plays", "tennis"),
    ("simon", "lives", "melbourne"),
    ("tennis", "sport", "melbourne"),
    ("melbourne", "located", "australia"),
    ("tennis", "plays", "simon"),
    ("melbourne", "lives", "simon"),
    ("melbourne", "sport", "tennis"),
    ("australia", "located", "melbourne"),
  ]);

  let graph = Graph::from(sro.as_ref());
  println!("Graph Nodes: {:?}", graph.nodes());
  println!("Grph Edges: {:?}", graph.edges());
  println!("\nAdj matrix:\n{:?}", graph.adj_matrix());

  let edge_features = graph.edge_features();
  println!("\nSparse edge features:\n{:?}", edge_features);
  println!("\nDense edge features:\n{:?}", edge_features.to_dense());

  // TODO: Model too large to fit in memory.
  // let path = "data/english-skipgram-mincount-50-ctx-10-ns-5-dims-300.fifu";
  // let mut reader = BufReader::new(File::open(path).unwrap());
  // let embed =
  //   Embeddings::<VocabWrap, StorageWrap>::read_embeddings(&mut reader).unwrap();
  // let result = embed.embedding("melbourne").unwrap();
  // println!("Embedding: {:?}", result);
}
