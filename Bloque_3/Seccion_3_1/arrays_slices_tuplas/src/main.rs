/*
Sección 3.1 - Arrays, slices y tuplas en la práctica

Descripción:
Este programa compara arrays, slices y tuplas usando datos de calificaciones.
También muestra cuándo conviene usar cada estructura y cómo acceder a sus valores.

Diagrama conceptual:
Array  -> colección de tamaño fijo con elementos del mismo tipo
Slice  -> vista prestada de una parte continua de una colección
Tupla  -> agrupación de valores que pueden tener tipos diferentes

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p arrays_slices_tuplas
*/

fn mostrar_slice(nombre: &str, valores: &[i32]) {
    println!("{nombre}: {valores:?}");
    println!("Cantidad de elementos: {}", valores.len());
}

fn main() {
    let calificaciones: [i32; 5] = [95, 88, 76, 100, 84];

    let primeras_tres = &calificaciones[0..3];
    let ultimas_dos = &calificaciones[3..5];

    mostrar_slice("Primeras tres calificaciones", primeras_tres);
    mostrar_slice("Últimas dos calificaciones", ultimas_dos);

    let perfil_estudiante: (&str, u8, bool) = ("Ana", 20, true);
    let (nombre, edad, curso_activo) = perfil_estudiante;

    println!("Nombre: {nombre}");
    println!("Edad: {edad}");
    println!("¿Curso activo?: {curso_activo}");
    println!("Nombre accedido desde la tupla: {}", perfil_estudiante.0);

    let meses = ["enero", "febrero", "marzo"];
    println!("Primer mes registrado: {}", meses[0]);
}

/*
Ejercicio autónomo:
1. Crea un array llamado temperaturas con siete valores i32.
2. Crea un slice con las primeras tres temperaturas.
3. Crea otro slice con las últimas cuatro temperaturas.
4. Crea una tupla con el nombre de una ciudad, el número de mediciones y si los datos son válidos.
5. Desestructura la tupla e imprime sus valores.
6. Usa la función mostrar_slice para imprimir ambos slices.
*/