/*
Sección 3.6 - Ejemplo práctico

Descripción:
Este programa integra arrays, slices, tuplas, String, &str, Vec<T>, HashMap<K, V>
y métodos con impl para procesar una lista pequeña de estudiantes.

Objetivo del programa:
1. Crear estudiantes con nombre y calificaciones.
2. Calcular promedios usando slices.
3. Guardar estudiantes en un Vec.
4. Crear un HashMap para consultar promedios por nombre.
5. Imprimir un reporte final.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p ejemplo_practico_bloque3
*/

use std::collections::HashMap;

struct Estudiante {
    nombre: String,
    calificaciones: Vec<u32>,
}

impl Estudiante {
    fn nuevo(nombre: &str, calificaciones: Vec<u32>) -> Self {
        Self {
            nombre: String::from(nombre),
            calificaciones,
        }
    }

    fn promedio(&self) -> f64 {
        calcular_promedio(&self.calificaciones)
    }

    fn resumen(&self) -> String {
        format!("{} tiene promedio {:.2}", self.nombre, self.promedio())
    }
}

fn calcular_promedio(calificaciones: &[u32]) -> f64 {
    let mut suma = 0;

    for calificacion in calificaciones {
        suma += calificacion;
    }

    suma as f64 / calificaciones.len() as f64
}

fn main() {
    let estudiantes = vec![
        Estudiante::nuevo("Ana", vec![95, 88, 91]),
        Estudiante::nuevo("Luis", vec![70, 84, 79]),
        Estudiante::nuevo("Sofía", vec![100, 98, 97]),
    ];

    let mut promedios: HashMap<String, f64> = HashMap::new();

    for estudiante in &estudiantes {
        println!("{}", estudiante.resumen());
        promedios.insert(estudiante.nombre.clone(), estudiante.promedio());
    }

    let consulta = "Sofía";

    match promedios.get(consulta) {
        Some(promedio) => println!("Promedio consultado de {consulta}: {promedio:.2}"),
        None => println!("No se encontró a {consulta}"),
    }

    let metadatos: (&str, usize) = ("Total de estudiantes", estudiantes.len());
    println!("{}: {}", metadatos.0, metadatos.1);
}

/*
Ejercicio autónomo:
1. Agrega un estudiante más al vector estudiantes.
2. Agrega un método llamado cantidad_calificaciones que regrese usize.
3. Modifica resumen para incluir cuántas calificaciones tiene cada estudiante.
4. Crea una función llamada mejor_promedio que reciba un slice de Estudiante y regrese el promedio más alto.
5. Imprime el mejor promedio al final del programa.
6. Mantén el código organizado usando funciones pequeñas y nombres claros.
*/