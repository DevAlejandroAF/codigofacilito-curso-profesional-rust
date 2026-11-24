/*
Sección 5.1 - structs

Descripción:
Este programa introduce los structs en Rust. Muestra cómo definirlos,
crear instancias, actualizar campos mutables y acceder a sus valores.

Diagrama conceptual:
struct Nombre {
    campo_1: Tipo,
    campo_2: Tipo,
}

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p structs
*/

struct Estudiante {
    nombre: String,
    matricula: u32,
    promedio: f64,
    activo: bool,
}

fn main() {
    let mut estudiante_1 = Estudiante {
        nombre: String::from("Ana"),
        matricula: 2025001,
        promedio: 9.4,
        activo: true,
    };

    println!("Nombre: {}", estudiante_1.nombre);
    println!("Matrícula: {}", estudiante_1.matricula);
    println!("Promedio inicial: {}", estudiante_1.promedio);
    println!("¿Activo?: {}", estudiante_1.activo);

    estudiante_1.promedio = 9.7;
    estudiante_1.activo = false;

    println!("Promedio actualizado: {}", estudiante_1.promedio);
    println!("¿Activo después de actualizar?: {}", estudiante_1.activo);

    let estudiante_2 = Estudiante {
        nombre: String::from("Luis"),
        matricula: 2025002,
        promedio: 8.8,
        activo: true,
    };

    println!("Segundo estudiante: {}", estudiante_2.nombre);
}

/*
Ejercicio autónomo:
1. Crea un struct llamado Libro con los campos titulo, autor, paginas y disponible.
2. Crea una instancia mutable de Libro con datos de ejemplo.
3. Imprime todos sus campos.
4. Modifica el valor de disponible.
5. Imprime nuevamente el estado actualizado del libro.
6. Agrega un comentario explicando por qué la variable del struct debe ser mutable para cambiar sus campos.
*/