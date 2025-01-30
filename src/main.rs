fn main() {
    let xy = (0, 0);
    match xy {
        (x @ 0, 0) => {
            println!("Este es el punto de origen. {x}")
        }
        (..=-1, 1..) => {
            println!("Este punto está en el segundo cuadrante.")
        }
        _ => {
            println!("Ni idea en que cuadrante estás.")
        }
    }
}
