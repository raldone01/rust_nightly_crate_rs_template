use crate::gen_hello_world;

#[test]
fn test_gen_hello_world() {
  assert_eq!("Hello World!", gen_hello_world());
}
