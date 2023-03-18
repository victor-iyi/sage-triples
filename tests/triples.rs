use sage_triples::Triple;

#[test]
fn test_triple() {
  let triple = Triple::new(
    "simon".to_string(),
    "plays".to_string(),
    "tennis".to_string(),
  );
  assert_eq!(triple.subject(), "simon");
  assert_eq!(triple.relation(), "plays");
  assert_eq!(triple.object(), "tennis");
}
