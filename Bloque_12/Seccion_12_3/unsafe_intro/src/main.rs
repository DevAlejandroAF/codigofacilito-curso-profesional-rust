/*
Sección 12.3 - Introducción a unsafe Rust

Descripción:
Este programa introduce unsafe Rust con raw pointers y una abstracción segura.
El objetivo es mostrar que unsafe no desactiva todas las reglas de Rust,
pero sí permite operaciones que el compilador no puede verificar completamente.

Diagrama conceptual:
Código seguro
    llama a función segura
        que contiene un bloque unsafe pequeño y revisado

Regla práctica:
Mantén unsafe lo más pequeño posible y crea una API segura alrededor.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p unsafe_intro
*/

fn leer_desde_raw_pointer(valor: &i32) -> i32 {
    let puntero: *const i32 = valor as *const i32;

    unsafe {
        *puntero
    }
}

fn intercambiar_extremos(datos: &mut [i32]) {
    if datos.len() < 2 {
        return;
    }

    let longitud = datos.len();
    let ptr = datos.as_mut_ptr();

    unsafe {
        let primero = ptr;
        let ultimo = ptr.add(longitud - 1);

        std::ptr::swap(primero, ultimo);
    }
}

fn main() {
    let numero = 42;
    let leido = leer_desde_raw_pointer(&numero);

    println!("Valor leído desde raw pointer: {leido}");

    let mut datos = [10, 20, 30, 40];
    println!("Antes de intercambiar: {datos:?}");

    intercambiar_extremos(&mut datos);
    println!("Después de intercambiar: {datos:?}");
}

/*
Ejercicio autónomo:
1. Crea una función segura llamada leer_primero_raw que reciba un slice de i32.
2. Si el slice está vacío, regresa None.
3. Si tiene datos, usa un raw pointer dentro de un bloque unsafe pequeño para leer el primer elemento.
4. La función debe regresar Option<i32>.
5. Prueba la función con un slice vacío y uno con datos.
6. Agrega un comentario explicando por qué la API pública puede ser segura aunque use unsafe internamente.
*/