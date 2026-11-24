/*
Sección 1.6 - Primer programa: Hello, world!

Descripción:
Este programa imprime mensajes en la terminal y sirve para explicar
la estructura mínima de un programa ejecutable en Rust.

Diagrama conceptual:
Código fuente
    |
    v
cargo check  -> revisa errores sin generar el ejecutable final
    |
    v
cargo build  -> compila y genera un binario en target/debug
    |
    v
cargo run    -> compila si hace falta y ejecuta el programa

Instrucciones:
Ejecuta desde la raíz CursoRust con:
cargo check -p hello_world
cargo build -p hello_world
cargo run -p hello_world
*/

fn main() {
    let mensaje = "Curso Rust";
    println!("Hello, world!");
    println!("Mi primer programa en Rust compiló correctamente.");
    print!("{mensaje}");
}

/*
Ejercicio autónomo:
1. Cambia el mensaje "Hello, world!" por un saludo personalizado.
2. Agrega una tercera línea que imprima el nombre del curso.
3. Ejecuta cargo check -p hello_world antes de ejecutar cargo run -p hello_world.
4. Observa si cargo check detecta errores antes de correr el programa.
*/

