/*
Sección 7.6 - Ejemplo práctico

Descripción:
Este programa integra lectura de archivos, parsing y manejo robusto de errores.
Procesa registros de estudiantes en formato nombre,calificacion.

Objetivo del programa:
1. Crear un archivo de ejemplo.
2. Leer el archivo como texto.
3. Parsear cada línea de forma segura.
4. Reportar registros válidos y errores por línea.
5. Continuar procesando aunque una línea falle.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p ejemplo_practico_errores
*/

use std::error::Error;
use std::fmt;
use std::fs;
use std::num::ParseIntError;

#[derive(Debug)]
struct Registro {
    nombre: String,
    calificacion: u32,
}

#[derive(Debug)]
enum RegistroError {
    FormatoInvalido,
    NombreVacio,
    CalificacionInvalida(ParseIntError),
    CalificacionFueraDeRango(u32),
}

impl fmt::Display for RegistroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistroError::FormatoInvalido => write!(f, "El formato debe ser nombre,calificacion."),
            RegistroError::NombreVacio => write!(f, "El nombre no puede estar vacío."),
            RegistroError::CalificacionInvalida(error) => write!(f, "La calificación no es numérica: {error}"),
            RegistroError::CalificacionFueraDeRango(valor) => {
                write!(f, "La calificación {valor} está fuera del rango 0..=100.")
            }
        }
    }
}

impl Error for RegistroError {}

impl From<ParseIntError> for RegistroError {
    fn from(error: ParseIntError) -> Self {
        RegistroError::CalificacionInvalida(error)
    }
}

fn parsear_registro(linea: &str) -> Result<Registro, RegistroError> {
    let (nombre, calificacion_texto) = linea
        .split_once(',')
        .ok_or(RegistroError::FormatoInvalido)?;

    let nombre = nombre.trim();

    if nombre.is_empty() {
        return Err(RegistroError::NombreVacio);
    }

    let calificacion = calificacion_texto.trim().parse::<u32>()?;

    if calificacion > 100 {
        return Err(RegistroError::CalificacionFueraDeRango(calificacion));
    }

    Ok(Registro {
        nombre: String::from(nombre),
        calificacion,
    })
}

fn leer_registros(ruta: &str) -> Result<Vec<Registro>, Box<dyn Error>> {
    let contenido = fs::read_to_string(ruta)?;
    let mut registros = Vec::new();

    for (indice, linea) in contenido.lines().enumerate() {
        match parsear_registro(linea) {
            Ok(registro) => registros.push(registro),
            Err(error) => println!("Línea {} ignorada: {}", indice + 1, error),
        }
    }

    Ok(registros)
}

fn main() -> Result<(), Box<dyn Error>> {
    let datos = "Ana,95\nLuis,82\n,70\nSofía,abc\nCarlos,120\nMaría,100";
    fs::write("calificaciones.txt", datos)?;

    let registros = leer_registros("calificaciones.txt")?;

    println!("Registros válidos:");
    for registro in registros {
        println!("{} obtuvo {}", registro.nombre, registro.calificacion);
    }

    Ok(())
}

/*
Ejercicio autónomo:
1. Agrega una columna extra al archivo de ejemplo con el grupo del estudiante: nombre,grupo,calificacion.
2. Actualiza el struct Registro para guardar el grupo.
3. Modifica parsear_registro para validar que existan tres columnas.
4. Valida que el grupo no esté vacío.
5. Mantén el comportamiento de continuar procesando aunque una línea falle.
6. Agrega un comentario explicando por qué este diseño es más robusto que usar unwrap en cada paso.
*/