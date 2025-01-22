# while

El bucle while en Rust es una herramienta poderosa para ejecutar código repetidamente mientras una condición sea verdadera. Es importante asegurarse de que la condición eventualmente se vuelva false

```rust
while condición {
// Código a ejecutar mientras la condición sea verdadera
}
```

# loop

es una estructura de control que crea un bucle infinito. A diferencia de while, que se ejecuta mientras una condición sea verdadera, loop se ejecuta indefinidamente hasta que se usa explícitamente la palabra clave break para salir del bucle.

```rust
loop {
    // Código a ejecutar repetidamente
    if condición {
        break; // Sale del bucle
    }
}
```
