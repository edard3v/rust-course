fn main() {
  let x = vec![1, 3, 4];

  let r: Vec<i32> = x.iter().map(|item| item * 2).collect();

  println!("{r:?}");
}
