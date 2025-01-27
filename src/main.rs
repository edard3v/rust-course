fn main() {
    let mut x = String::from("¡Hola");

    {
        let y = &mut x;
        y.push_str(" mundo!");
    }

    println!("{x}");
}
