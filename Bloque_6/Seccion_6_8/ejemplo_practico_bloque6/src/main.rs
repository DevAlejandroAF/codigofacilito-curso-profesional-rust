/*
Sección 6.8 - Ejemplo práctico

Descripción:
Este programa integra generics, traits, closures, iterators y lifetimes
para procesar una colección de tareas de estudio.

Objetivo del programa:
1. Modelar tareas con un struct.
2. Usar traits para describir elementos.
3. Usar generics y trait bounds para imprimir reportes.
4. Usar closures e iterators para filtrar y transformar datos.
5. Usar lifetimes para regresar referencias válidas a datos existentes.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p ejemplo_practico_bloque6
*/

trait Describible {
    fn describir(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
struct Tarea {
    titulo: String,
    completada: bool,
    prioridad: u8,
}

impl Describible for Tarea {
    fn describir(&self) -> String {
        let estado = if self.completada { "completada" } else { "pendiente" };

        format!("{} - prioridad {} - {}", self.titulo, self.prioridad, estado)
    }
}

fn imprimir_reporte<T>(elementos: &[T])
where
    T: Describible,
{
    for elemento in elementos {
        println!("{}", elemento.describir());
    }
}

fn filtrar_tareas<F>(tareas: &[Tarea], criterio: F) -> Vec<&Tarea>
where
    F: Fn(&Tarea) -> bool,
{
    tareas.iter().filter(|tarea| criterio(tarea)).collect()
}

fn tarea_mayor_prioridad<'a>(tareas: &'a [Tarea]) -> Option<&'a Tarea> {
    tareas.iter().max_by_key(|tarea| tarea.prioridad)
}

fn main() {
    let tareas = vec![
        Tarea {
            titulo: String::from("Leer capítulo de generics"),
            completada: true,
            prioridad: 2,
        },
        Tarea {
            titulo: String::from("Practicar traits"),
            completada: false,
            prioridad: 5,
        },
        Tarea {
            titulo: String::from("Resolver ejercicio de iterators"),
            completada: false,
            prioridad: 4,
        },
    ];

    println!("Reporte completo:");
    imprimir_reporte(&tareas);

    let pendientes = filtrar_tareas(&tareas, |tarea| !tarea.completada);

    println!("Tareas pendientes:");
    for tarea in &pendientes {
        println!("{}", tarea.describir());
    }

    let titulos_importantes: Vec<String> = tareas
        .iter()
        .filter(|tarea| tarea.prioridad >= 4)
        .map(|tarea| tarea.titulo.clone())
        .collect();

    println!("Títulos importantes: {titulos_importantes:?}");

    match tarea_mayor_prioridad(&tareas) {
        Some(tarea) => println!("Mayor prioridad: {}", tarea.describir()),
        None => println!("No hay tareas registradas."),
    }
}

/*
Ejercicio autónomo:
1. Agrega una tarea más al vector tareas.
2. Crea un trait llamado Priorizable con un método prioridad que regrese u8.
3. Implementa Priorizable para Tarea.
4. Crea una función genérica que reciba un slice de elementos Priorizable y cuente cuántos tienen prioridad alta.
5. Usa iterators para obtener los títulos de tareas completadas.
6. Agrega un comentario explicando cómo se combinan generics, traits, closures, iterators y lifetimes en este ejemplo.
*/