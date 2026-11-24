/*
Sección 1.5 - Estructura básica de un proyecto Rust

Descripción:
Este archivo representa una librería dentro del proyecto.
Aquí colocamos funciones reutilizables que pueden ser llamadas desde main.rs.

Instrucciones:
Este archivo debe estar dentro de la carpeta src.
*/

pub fn nombre_del_curso() -> &'static str {
    "Curso de Rust"
}

pub fn mensaje_de_bienvenida() -> String {
    format!("Bienvenida al {}", nombre_del_curso())
}