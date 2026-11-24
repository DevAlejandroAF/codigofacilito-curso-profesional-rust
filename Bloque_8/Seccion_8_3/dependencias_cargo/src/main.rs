/*
Sección 8.3 - Dependencias con Cargo

Descripción:
Este programa usa serde y serde_json como dependencias externas.
Muestra cómo convertir un struct de Rust a JSON y cómo leer JSON de vuelta.

Diagrama conceptual:
Cargo.toml
└── [dependencies]
    ├── serde      -> serialización y deserialización
    └── serde_json -> trabajo con formato JSON

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p dependencias_cargo
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Curso {
    nombre: String,
    horas: u32,
    activo: bool,
}

fn main() {
    let curso = Curso {
        nombre: String::from("Rust organizado con Cargo"),
        horas: 12,
        activo: true,
    };

    let json = serde_json::to_string_pretty(&curso).expect("El curso debería serializarse.");
    println!("Curso como JSON:");
    println!("{json}");

    let curso_recuperado: Curso =
        serde_json::from_str(&json).expect("El JSON debería convertirse a Curso.");

    println!("Curso recuperado: {curso_recuperado:?}");
}

/*
Ejercicio autónomo:
1. Agrega un campo nivel: String al struct Curso.
2. Actualiza la instancia inicial para incluir ese campo.
3. Ejecuta el programa y observa cómo cambia el JSON.
4. Crea otro struct llamado Estudiante con nombre y promedio.
5. Serializa una instancia de Estudiante a JSON.
6. Agrega un comentario explicando qué dependency usa features y para qué sirve derive.
*/