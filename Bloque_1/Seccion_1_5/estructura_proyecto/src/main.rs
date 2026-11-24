/*
Sección 1.5 - Estructura básica de un proyecto Rust

Descripción:
Este archivo representa el ejecutable principal del proyecto.
Usa una función definida en lib.rs para mostrar cómo un proyecto puede
separar la lógica reutilizable del punto de entrada del programa.

Instrucciones:
Ejecuta el proyecto desde la raíz CursoRust con:
cargo run -p estructura_proyecto
*/

use estructura_proyecto::mensaje_de_bienvenida;

fn main() {
    let mensaje = mensaje_de_bienvenida();
    println!("{mensaje}");
}