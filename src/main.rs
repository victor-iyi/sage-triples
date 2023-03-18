use sage_triples::Graph;
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

  let graph = Graph::from(sro);
  println!("Graph: {:?}", graph);
  println!("Adj matrix: {:?}", graph.adj_matrix());
  // println!("Hello, world!");
}
