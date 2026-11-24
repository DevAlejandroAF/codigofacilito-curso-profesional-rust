/*
Sección 8.2 - Packages y crates

Descripción:
Este archivo define el crate de librería del package.
La librería contiene lógica reutilizable que puede ser usada por varios binarios.

Diagrama conceptual:
package packages_crates
├── library crate: src/lib.rs
├── binary crate: src/main.rs
└── binary crate: src/bin/reporte.rs
*/

pub fn formatear_titulo(titulo: &str) -> String {
    format!("=== {titulo} ===")
}

pub fn contar_palabras(texto: &str) -> usize {
    texto.split_whitespace().count()
}

/*
Ejercicio autónomo:
1. Agrega una función pública llamada convertir_a_mayusculas.
2. La función debe recibir &str y regresar String.
3. Usa esa función desde src/main.rs.
4. Crea un segundo archivo dentro de src/bin llamado resumen.rs.
5. Desde ese nuevo binario, usa al menos una función de la librería.
6. Agrega un comentario explicando qué crate es librería y cuáles son binarios.
*/