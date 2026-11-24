/*
Sección 9.5 - Introducción al diseño de herramientas CLI

Descripción:
Este archivo representa la capa ejecutable de una herramienta CLI.
La lógica de parsing y ejecución vive en lib.rs para mantener main pequeño.

Diagrama conceptual:
main.rs -> obtiene argumentos y muestra errores
lib.rs  -> define configuración, ayuda y ejecución del comando

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p diseno_cli -- --help
cargo run -p diseno_cli -- saludar Ana
*/

use diseno_cli::{ejecutar, Config};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match Config::desde_args(&args) {
        Ok(config) => ejecutar(config),
        Err(error) => {
            eprintln!("Error: {error}");
            println!("{}", Config::ayuda());
        }
    }
}