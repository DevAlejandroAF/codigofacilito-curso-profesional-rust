/*
Sección 7.5 - Errores personalizados

Descripción:
Este programa muestra cómo definir un tipo de error propio usando enum.
También implementa Display, Error y From para integrarlo con Result y el operador ?.

Diagrama conceptual:
enum AppError {
    VarianteUno,
    VarianteDos(Dato),
}

impl Display -> mensaje legible para personas
impl Error   -> integración con el ecosistema de errores
impl From    -> conversión automática desde otro error

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p errores_personalizados
*/

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    CampoVacio,
    NumeroInvalido(ParseIntError),
    FueraDeRango { valor: i32, minimo: i32, maximo: i32 },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::CampoVacio => write!(f, "El campo no puede estar vacío."),
            AppError::NumeroInvalido(error) => write!(f, "No se pudo convertir el número: {error}"),
            AppError::FueraDeRango { valor, minimo, maximo } => {
                write!(f, "El valor {valor} está fuera del rango {minimo}..={maximo}.")
            }
        }
    }
}

impl Error for AppError {}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::NumeroInvalido(error)
    }
}

fn parsear_puntaje(texto: &str) -> Result<i32, AppError> {
    if texto.trim().is_empty() {
        return Err(AppError::CampoVacio);
    }

    let valor = texto.trim().parse::<i32>()?;

    if !(0..=100).contains(&valor) {
        return Err(AppError::FueraDeRango {
            valor,
            minimo: 0,
            maximo: 100,
        });
    }

    Ok(valor)
}

fn main() {
    for entrada in ["95", "", "abc", "150"] {
        match parsear_puntaje(entrada) {
            Ok(puntaje) => println!("Puntaje válido: {puntaje}"),
            Err(error) => println!("Entrada '{entrada}' inválida: {error}"),
        }
    }
}

/*
Ejercicio autónomo:
1. Crea un enum llamado RegistroError con variantes NombreVacio, EdadInvalida y EdadFueraDeRango.
2. Implementa fmt::Display para mostrar mensajes claros.
3. Implementa std::error::Error para RegistroError.
4. Implementa From<std::num::ParseIntError> para convertir errores de parseo.
5. Crea una función validar_edad que reciba &str y regrese Result<u8, RegistroError>.
6. Prueba la función con entradas válidas, vacías, no numéricas y fuera de rango.
*/