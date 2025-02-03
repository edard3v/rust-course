fn main() {
  let x = "edar".to_string();
  let y = "d3v".to_string();

  longest(x.as_str(), y.as_str());
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
  if a.len() > b.len() {
    a
  } else {
    b
  }
}
