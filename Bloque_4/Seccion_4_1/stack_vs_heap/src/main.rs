/*
Sección 4.1 - Stack vs Heap

Descripción:
Este programa muestra diferencias prácticas entre datos almacenados directamente
en el stack y datos que administran memoria en el heap, usando enteros, arrays y String.

Diagrama conceptual:
Stack
    - Guarda datos de tamaño conocido en tiempo de compilación.
    - Funciona como LIFO: last in, first out.
    - Suele ser rápido para insertar y retirar valores.

Heap
    - Guarda datos cuyo tamaño puede cambiar o no se conoce completamente al compilar.
    - Requiere pedir memoria al allocator.
    - Rust libera esa memoria automáticamente cuando el owner sale de scope.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p stack_vs_heap
*/

fn imprimir_info_stack(numero: i32, arreglo: [i32; 3]) {
    println!("Número recibido: {numero}");
    println!("Arreglo recibido: {arreglo:?}");
}

fn imprimir_info_heap(texto: &String) {
    println!("Texto recibido: {texto}");
    println!("Longitud en bytes: {}", texto.len());
    println!("Capacidad actual: {}", texto.capacity());
}

fn main() {
    let numero = 42;
    let arreglo = [10, 20, 30];

    imprimir_info_stack(numero, arreglo);

    let mut mensaje = String::from("Rust");
    mensaje.push_str(" maneja memoria con ownership");

    imprimir_info_heap(&mensaje);

    println!("Número después de la función: {numero}");
    println!("Arreglo después de la función: {arreglo:?}");
    println!("Mensaje después de la función: {mensaje}");
}

/*
Ejercicio autónomo:
1. Crea una variable entera y un array de cuatro enteros.
2. Crea una función que reciba esos valores y los imprima.
3. Crea un String mutable con una frase corta.
4. Agrega más texto al String usando push_str.
5. Crea una función que reciba una referencia al String e imprima su longitud y capacidad.
6. Explica en un comentario por qué el String puede crecer y el array no.
*/