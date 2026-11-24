/*
Sección 9.6 - Ejemplo práctico

Descripción:
Este archivo es el punto de entrada de una utilidad CLI para registrar tareas.
La lógica de negocio vive en lib.rs y main.rs se encarga de conectar argumentos,
ejecución y mensajes de error.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p utilidad_cli -- agregar "Estudiar Rust"
cargo run -p utilidad_cli -- listar
cargo run -p utilidad_cli -- completar 1
*/

use std::error::Error;
use utilidad_cli::{ejecutar, Config};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::desde_args(&args)?;

    ejecutar(config)?;

    Ok(())
}