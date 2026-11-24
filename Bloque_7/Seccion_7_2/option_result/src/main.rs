/*
Sección 7.2 - Option<T> y Result<T, E>

Descripción:
Este programa compara Option<T> y Result<T, E>.
Option representa ausencia de valor, mientras que Result representa éxito o fallo con información de error.

Diagrama conceptual:
Option<T>
    Some(valor) -> hay valor
    None        -> no hay valor

Result<T, E>
    Ok(valor)   -> operación exitosa
    Err(error)  -> operación fallida con explicación

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p option_result
*/

fn buscar_usuario(id: u32) -> Option<&'static str> {
    match id {
        1 => Some("Ana"),
        2 => Some("Luis"),
        _ => None,
    }
}

fn convertir_a_numero(texto: &str) -> Result<i32, String> {
    match texto.parse::<i32>() {
        Ok(numero) => Ok(numero),
        Err(_) => Err(format!("'{texto}' no es un número válido.")),
    }
}

fn calcular_descuento(precio: f64, porcentaje: f64) -> Result<f64, String> {
    if precio < 0.0 {
        Err(String::from("El precio no puede ser negativo."))
    } else if !(0.0..=100.0).contains(&porcentaje) {
        Err(String::from("El porcentaje debe estar entre 0 y 100."))
    } else {
        Ok(precio * porcentaje / 100.0)
    }
}

fn main() {
    match buscar_usuario(1) {
        Some(nombre) => println!("Usuario encontrado: {nombre}"),
        None => println!("No se encontró el usuario."),
    }

    match buscar_usuario(99) {
        Some(nombre) => println!("Usuario encontrado: {nombre}"),
        None => println!("No se encontró el usuario."),
    }

    match convertir_a_numero("42") {
        Ok(numero) => println!("Número convertido: {numero}"),
        Err(error) => println!("Error al convertir: {error}"),
    }

    match calcular_descuento(500.0, 15.0) {
        Ok(descuento) => println!("Descuento aplicado: ${descuento:.2}"),
        Err(error) => println!("Error al calcular descuento: {error}"),
    }
}

/*
Ejercicio autónomo:
1. Crea una función llamada buscar_producto que reciba un código u32 y regrese Option<&'static str>.
2. Crea una función llamada convertir_precio que reciba &str y regrese Result<f64, String>.
3. Crea una función llamada validar_stock que reciba i32 y regrese Result<i32, String>.
4. Usa match en main para manejar Some, None, Ok y Err.
5. Prueba al menos un caso exitoso y un caso fallido por cada tipo.
6. Agrega un comentario explicando cuándo usarías Option y cuándo usarías Result.
*/