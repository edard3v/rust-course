fn main() {
  let x = "EDAR".to_string();
  let y = &x;

  foo(y);
}

fn foo(a: &str) -> &str {
  a
}
