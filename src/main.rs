fn main() {
  let x = divide(4, 0);

  match x {
    Err(x) => println!("{x}"),
    Ok(x) => println!("{x}"),
  }
}

fn divide(a: u32, b: u32) -> Result<u32, String> {
  if b == 0 {
    Err("No es posible dividir por 0".to_string())
  } else {
    Ok(a / b)
  }
}
