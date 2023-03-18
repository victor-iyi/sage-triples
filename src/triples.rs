/// Representation of a triple in the Knowledge Graph.
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
    write!(f, "({:?} -- {:?} -- {:?})", self.0, self.1, self.2)
  }
}

impl std::fmt::Display for Triple {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({} -- {} -- {})", self.0, self.1, self.2)
  }
}
