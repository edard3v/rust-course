fn main() {
    let mut a = vec![1, 2];

    println!("{a:?}");

    a.insert(1, 9);
    println!("{a:?}");

    a.pop();
    println!("{a:?}");

    a.remove(1);
    println!("{a:?}");
}
