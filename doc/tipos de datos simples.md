# Tipos de Datos Simples en Rust

## **1. Tipos Numéricos**

### **Enteros (`i*` y `u*`)**

- Representan números enteros, con y sin signo.
- Tamaños disponibles: 8, 16, 32, 64, 128 bits, y un tamaño dependiente de la arquitectura (`isize`/`usize`).

| Tipo    | Tamaño                     | Rango                                                                                                      |
| ------- | -------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `i8`    | 8 bits                     | -128 a 127                                                                                                 |
| `u8`    | 8 bits                     | 0 a 255                                                                                                    |
| `i16`   | 16 bits                    | -32,768 a 32,767                                                                                           |
| `u16`   | 16 bits                    | 0 a 65,535                                                                                                 |
| `i32`   | 32 bits                    | -2,147,483,648 a 2,147,483,647                                                                             |
| `u32`   | 32 bits                    | 0 a 4,294,967,295                                                                                          |
| `i64`   | 64 bits                    | -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807                                                     |
| `u64`   | 64 bits                    | 0 a 18,446,744,073,709,551,615                                                                             |
| `i128`  | 128 bits                   | -170,141,183,460,469,231,731,687,303,715,884,105,728 a 170,141,183,460,469,231,731,687,303,715,884,105,727 |
| `u128`  | 128 bits                   | 0 a 340,282,366,920,938,463,463,374,607,431,768,211,455                                                    |
| `isize` | Depende de la arquitectura | Depende del sistema operativo (32 o 64 bits)                                                               |
| `usize` | Depende de la arquitectura | Similar a `isize`, pero sin signo                                                                          |

---

### **Punto Flotante (`f32`, `f64`)**

- Representan números decimales.
- Compatibles con IEEE 754.

| Tipo  | Tamaño  | Precisión        |
| ----- | ------- | ---------------- |
| `f32` | 32 bits | Precisión simple |
| `f64` | 64 bits | Precisión doble  |

---

## **2. Tipo Booleano**

- **`bool`**
  - Representa valores booleanos.
  - Valores posibles: `true` o `false`.
  - Tamaño: 1 bit.

---

## **3. Tipo de Carácter**

- **`char`**
  - Representa un único carácter Unicode.
  - Tamaño: 4 bytes (32 bits).
  - Ejemplo:
    ```rust
    let c: char = 'a';
    let emoji: char = '😊';
    ```

---

## **4. Tipos de Referencia a Datos Simples**

- **`&str`**
  - Representa una secuencia de texto inmutable (cadena de caracteres).
  - Usada principalmente para texto de tamaño fijo.
  - Ejemplo:
    ```rust
    let texto: &str = "Hola, mundo!";
    ```

---
