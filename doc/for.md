# Bucles For en Rust

- El rango `0..5` es exclusivo (0 a 4)
- El rango `0..=5` es inclusivo (0 a 5)
- Se puede usar `break` y `continue`
- `for` en Rust es seguro y eficiente

## Sintaxis Básica

```rust
for numero in 0..5 {
    println!("Número: {}", numero);
}
```

## Iteración Inversa

```rust
for numero in (0..5).rev() {
    println!("Countdown: {}", numero);
}
```

## Range con Step

```rust
// Cuenta de 2 en 2
for i in (0..10).step_by(2) {
    println!("{}", i);
}
```

## Iterando sobre Arrays

```rust
let numeros = [1, 2, 3, 4, 5];

for n in numeros.iter() {
    println!("Valor: {}", n);
}
```

## Iteración con Enumerate

```rust
let frutas = vec!["manzana", "pera", "plátano"];

for (indice, fruta) in frutas.iter().enumerate() {
    println!("{}: {}", indice, fruta);
}
```
