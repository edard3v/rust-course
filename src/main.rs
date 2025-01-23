fn main() {
    let x = add(5.5, 6.0);
    let y = subtract(5.5, 6.0);

    println!("{x} {y}");
}

fn add(a: f64, b: f64) -> f64 {
    return a + b;
}

fn subtract(a: f64, b: f64) -> f64 {
    return a - b;
}
