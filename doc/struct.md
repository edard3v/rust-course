# Struct

Permiten agrupar varios campos relacionados en un solo tipo, lo que es útil para organizar y manipular datos de manera eficiente.

```rust
struct Person {
    username: String,
    age: u8,
}
```

## Crear instancias

```rust
let edar = Person {
    username: String::from("edard3v"),
    age: 30,
};
```
