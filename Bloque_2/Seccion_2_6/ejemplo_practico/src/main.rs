/*
Sección 2.6 - Ejemplo práctico

Descripción:
Este programa integra variables, constantes, tipos de datos, funciones,
condicionales y ciclos para analizar calificaciones de un estudiante.

Objetivo del programa:
1. Guardar varias calificaciones en un array.
2. Calcular el promedio usando un ciclo for.
3. Determinar si el estudiante aprueba.
4. Mostrar un resumen final.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p ejemplo_practico
*/

const CALIFICACION_MINIMA: f64 = 70.0;

fn calcular_promedio(calificaciones: [f64; 5]) -> f64 {
    let mut suma = 0.0;

    for calificacion in calificaciones {
        suma += calificacion;
    }

    suma / 5.0
}

fn obtener_estado(promedio: f64) -> &'static str {
    if promedio >= CALIFICACION_MINIMA {
        "Aprobado"
    } else {
        "No aprobado"
    }
}

fn main() {
    let nombre = "Luis";
    let calificaciones: [f64; 5] = [85.0, 92.5, 76.0, 68.5, 90.0];

    let promedio = calcular_promedio(calificaciones);
    let estado = obtener_estado(promedio);

    println!("Resumen académico");
    println!("Estudiante: {nombre}");
    println!("Promedio: {promedio}");
    println!("Estado: {estado}");

    let mensaje = if promedio >= 90.0 {
        "Desempeño sobresaliente"
    } else if promedio >= 80.0 {
        "Buen desempeño"
    } else if promedio >= CALIFICACION_MINIMA {
        "Desempeño suficiente"
    } else {
        "Necesita reforzar temas"
    };

    println!("Comentario: {mensaje}");
}

/*
Ejercicio autónomo:
1. Cambia el nombre del estudiante.
2. Modifica el array para que tenga cinco calificaciones distintas.
3. Crea una constante llamada CALIFICACION_EXCELENTE con valor 90.0.
4. Usa esa constante dentro del if que genera el mensaje final.
5. Agrega una función llamada contar_aprobadas que reciba un array de cinco f64 y regrese cuántas calificaciones son mayores o iguales a 70.0.
6. Imprime cuántas calificaciones aprobadas tiene el estudiante.
*/