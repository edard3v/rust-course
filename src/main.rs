fn main() {
  let x = vec![1, 2, 3];

  let r = x.iter().map(|item| item * 2);
  let r2 = x.iter().filter(|item| *item % 2 == 2);

  println!("{r:?} {r2:?}");
}
