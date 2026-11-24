/*
Sección 2.2 - Tipos de datos

Descripción:
Este programa muestra los tipos escalares y compuestos básicos en Rust.
Se trabajan enteros, flotantes, booleanos, caracteres, tuplas y arrays.

Diagrama conceptual:
Tipos escalares  -> representan un solo valor
    - enteros
    - flotantes
    - booleanos
    - caracteres

Tipos compuestos -> agrupan varios valores
    - tuplas
    - arrays

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p tipos_datos
*/

fn main() {
    let edad: u8 = 22;
    let temperatura: i32 = -5;
    let poblacion: u64 = 1_000_000;

    let promedio: f64 = 9.75;
    let porcentaje: f32 = 87.5;

    let curso_activo: bool = true;
    let inicial: char = 'R';
    let letra_especial: char = 'ñ';

    let estudiante: (&str, u8, f64) = ("Ana", 20, 9.4);
    let (nombre, edad_estudiante, promedio_estudiante) = estudiante;

    let calificaciones: [u8; 4] = [95, 88, 76, 100];
    let primera_calificacion = calificaciones[0];

    println!("Edad: {edad}");
    println!("Temperatura: {temperatura}");
    println!("Población: {poblacion}");
    println!("Promedio general: {promedio}");
    println!("Porcentaje: {porcentaje}");
    println!("¿Curso activo?: {curso_activo}");
    println!("Inicial del lenguaje: {inicial}");
    println!("Letra especial: {letra_especial}");

    println!(
        "Estudiante: {nombre}, edad: {edad_estudiante}, promedio: {promedio_estudiante}"
    );

    println!("Primera calificación: {primera_calificacion}");
    println!("Total de calificaciones registradas: {}", calificaciones.len());
}

/*
Ejercicio autónomo:
1. Crea una variable de tipo i16 para representar una temperatura negativa.
2. Crea una variable de tipo f64 para representar un precio con decimales.
3. Crea una variable booleana que indique si un usuario está registrado.
4. Crea una tupla con el nombre de un producto, su precio y si está disponible.
5. Desestructura la tupla en variables individuales.
6. Crea un array con cinco números enteros e imprime su primer y último elemento.
*/