# Vectores en Rust (Vec<T>)

## Definición

Un vector en Rust es una colección dinámica y mutable que:

- Almacena elementos del mismo tipo (T)
- Tiene un tamaño dinámico que puede crecer o disminuir en tiempo de ejecución

## Creación de Vectores

### Métodos de Creación

1. **Vec::new()**

```rust
let mut v: Vec<i32> = Vec::new(); // Vector vacío de enteros
```

2. **Macro vec!**

```rust
let v = vec![1, 2, 3]; // Vector con valores iniciales: [1, 2, 3]
```

3. **Con capacidad inicial**

```rust
let mut v = Vec::with_capacity(10); // Vector con capacidad inicial de 10 elementos
```

## Operaciones Básicas

### Añadir Elementos

```rust
let mut v = vec![1, 2, 3];
v.push(4); // Añade el valor 4 al final
```

### Acceder a Elementos

#### Usando Índices

```rust
let v = vec![1, 2, 3];
let tercer_elemento = v[2]; // Accede al tercer elemento (índice 2)
```

#### Usando get() (Seguro)

```rust
let v = vec![1, 2, 3];
match v.get(2) {
    Some(tercer_elemento) => println!("El tercer elemento es: {}", tercer_elemento),
    None => println!("No hay tercer elemento"),
}
```

### Eliminar Elementos

```rust
let mut v = vec![1, 2, 3];
let ultimo_elemento = v.pop(); // Elimina y devuelve el último elemento
```

### Longitud y Capacidad

```rust
let v = vec![1, 2, 3];
println!("Longitud: {}", v.len()); // Imprime: 3
println!("Capacidad: {}", v.capacity()); // Imprime la capacidad actual
```

## Modificación Avanzada

### Insertar Elemento

```rust
let mut v = vec![1, 2, 3];
v.insert(1, 10); // Inserta el valor 10 en la posición 1
```

### Eliminar Elemento en Posición Específica

```rust
let mut v = vec![1, 2, 3];
v.remove(1); // Elimina el elemento en la posición 1
```

### Vaciar Vector

```rust
let mut v = vec![1, 2, 3];
v.clear(); // Vacía el vector
```
