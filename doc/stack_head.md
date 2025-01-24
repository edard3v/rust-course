# Stack y Heap: Bases de la Memoria

## **Stack (Pila)**

- Es una región de memoria **rápida y organizada**.
- Se usa para almacenar datos con un **tamaño fijo y conocido en tiempo de compilación**.
- La memoria se asigna y libera automáticamente cuando las variables salen de su ámbito.

## **Heap (Montículo)**

- Es una región de memoria **más lenta pero flexible**.
- Se usa para almacenar datos con un **tamaño dinámico o desconocido en tiempo de compilación**.
- La memoria no se libera automáticamente; en Rust, se gestiona mediante el sistema de propiedad.

## **¿Por qué es importante entender stack y heap?**

- Rust usa el **stack** para datos pequeños y de tamaño fijo, y el **heap** para datos grandes o dinámicos.
- Entender la diferencia te ayudará a comprender cómo Rust gestiona la memoria y por qué ciertas operaciones son más eficientes que otras.

## **Conclusión**

| **Característica**     | **Stack**                                  | **Heap**                                             |
| ---------------------- | ------------------------------------------ | ---------------------------------------------------- |
| **Velocidad**          | Muy rápida                                 | Más lenta                                            |
| **Tamaño**             | Fijo (conocido en tiempo de compilación)   | Dinámico (puede cambiar en tiempo de ejecución)      |
| **Gestión de memoria** | Automática (se libera al salir del ámbito) | Automática en Rust (gracias al sistema de propiedad) |
| **Uso típico**         | Variables locales, datos pequeños          | Datos grandes o dinámicos                            |
