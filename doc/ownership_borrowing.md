# Gestion de memoria

Rust destaca por su sistema de gestión de memoria seguro y eficiente, sin necesidad de un recolector de basura. Esto se logra a través de dos conceptos clave: ownership (propiedad) y borrowing (préstamo).

## Ownership (Propiedad)

Es la base de la gestión de memoria en Rust. Las reglas de ownership garantizan que la memoria se maneje de manera segura, evitando problemas comunes como desbordamientos de memoria o accesos inválidos.

### Reglas

1.  Cada valor en Rust tiene un dueño (owner).

    - El dueño es la variable que posee el valor.
    - Solo puede haber un dueño a la vez.

2.  Cuando el dueño sale del ámbito (scope), el valor se libera.

    - Rust llama automáticamente a **drop** para liberar la memoria cuando la variable dueña sale del ámbito.

3.  El ownership puede ser transferido.

    - Cuando asignas un valor a otra variable o lo pasas como argumento a una función, el ownership se transfiere.
    - La variable original ya no puede acceder al valor.

## Borrowing (Préstamo)

En lugar de transferir el ownership, Rust permite prestar (borrow) un valor. Esto se hace mediante referencias (&).

### Reglas

1.  Puedes tener una o más referencias inmutables, o una única referencia mutable, pero no ambas al mismo tiempo.
2.  Las referencias deben ser válidas. Rust garantiza que las referencias no apunten a memoria liberada (dangling references).

## ¿Cuándo Termina el Préstamo?

En dos situaciones:

- Cuando la referencia sale del ámbito. Por ejemplo, si una referencia se crea dentro de un bloque, el préstamo termina cuando el bloque finaliza.

- Cuando la referencia deja de usarse.
