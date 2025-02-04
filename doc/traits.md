# Traits en Rust

Permiten definir funcionalidades comunes que pueden ser compartidas entre diferentes tipos, promoviendo la reutilización de código y la abstracción. En Rust, los traits son una herramienta poderosa para lograr **polimorfismo** sin sacrificar el rendimiento o la seguridad de tipos.

## Beneficios

1. **Reutilización de código**

2. **Polimorfismo**: Los traits permiten escribir funciones genéricas que pueden operar sobre cualquier tipo que implemente un trait específico.

3. **Abstracción**: Los traits permiten ocultar los detalles de implementación, exponiendo solo la interfaz necesaria para interactuar con un tipo.

4. **Composición sobre herencia**: Rust no tiene herencia tradicional, pero los traits permiten componer comportamientos de manera modular, lo que es más flexible y seguro.

5. **Seguridad de tipos**: Los traits en Rust se verifican en tiempo de compilación, lo que garantiza que los tipos implementen correctamente los métodos requeridos.

## Ejemplo básico

```rust
fn main() {
  trait Printable {
    fn print(&self, txt: &str) {
      println!("{txt}");
    }
  }

  struct Animal;
  impl Printable for Animal {}

  struct Account;
  impl Printable for Account {}

  let x = Animal {};
  let y = Account {};

  x.print("Animal");
  y.print("Account");
}
```
