fn main() {
  let x = |a, b| a + b;

  fn y<F>(f: F) -> i32
  where
    F: Fn(i32, i32) -> i32,
  {
    f(2, 3)
  }

  println!("{}", y(x))
}
