/*
Sección 2.1 - Variables, mutabilidad, shadowing y constantes

Descripción:
Este programa muestra cómo declarar variables con let, cómo hacerlas mutables con mut,
cómo usar constantes con const y cómo funciona el shadowing en Rust.

Diagrama conceptual:
let nombre = valor;          -> variable inmutable por defecto
let mut nombre = valor;      -> variable mutable
const NOMBRE: Tipo = valor;  -> constante, siempre requiere tipo explícito
let nombre = nuevo_valor;    -> shadowing, crea una nueva variable con el mismo nombre

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p variables_mutabilidad
*/

const PUNTAJE_MAXIMO: u32 = 100;

fn main() {
    let nombre = "Maeva";
    println!("Nombre inicial: {nombre}");

    let mut intentos = 1;
    println!("Intentos iniciales: {intentos}");

    intentos += 1;
    println!("Intentos después de actualizar: {intentos}");

    let puntaje = 85;
    let puntaje = puntaje + 10;
    let puntaje = format!("{puntaje} puntos de {PUNTAJE_MAXIMO}");
    println!("Puntaje final: {puntaje}");

    {
        let nombre = "Estudiante de Rust";
        println!("Nombre dentro del bloque interno: {nombre}");
    }

    println!("Nombre fuera del bloque interno: {nombre}");
}

/*
Ejercicio autónomo:
1. Crea una constante llamada LIMITE_INTENTOS con valor 3.
2. Crea una variable mutable llamada intentos_restantes con valor inicial LIMITE_INTENTOS.
3. Réstale 1 a intentos_restantes e imprime el resultado.
4. Crea una variable llamada mensaje con un texto corto.
5. Usa shadowing para convertir mensaje en su longitud usando .len().
6. Imprime la longitud final.
*/
