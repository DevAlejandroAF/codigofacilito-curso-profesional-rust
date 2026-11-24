/*
Sección 3.5 - Introducción a estilo idiomático en Rust

Descripción:
Este programa muestra primeras convenciones de estilo idiomático en Rust:
nombres claros, snake_case, uso de funciones pequeñas y preferencia por iteradores simples.

Diagrama conceptual:
Código idiomático -> código que sigue convenciones comunes del lenguaje
Legibilidad       -> nombres claros, funciones pequeñas y flujo fácil de seguir
Convención        -> acuerdo de estilo que facilita leer código de otras personas

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p estilo_idiomatico
*/

const CALIFICACION_MINIMA: u32 = 70;

fn es_aprobatoria(calificacion: u32) -> bool {
    calificacion >= CALIFICACION_MINIMA
}

fn contar_aprobadas(calificaciones: &[u32]) -> usize {
    let mut total = 0;

    for calificacion in calificaciones {
        if es_aprobatoria(*calificacion) {
            total += 1;
        }
    }

    total
}

fn imprimir_reporte(nombre_estudiante: &str, calificaciones: &[u32]) {
    let aprobadas = contar_aprobadas(calificaciones);
    let total = calificaciones.len();

    println!("Reporte de {nombre_estudiante}");
    println!("Calificaciones aprobatorias: {aprobadas} de {total}");
}

fn main() {
    let nombre_estudiante = "Sofía";
    let calificaciones = [90, 65, 78, 100, 58];

    imprimir_reporte(nombre_estudiante, &calificaciones);
}

/*
Ejercicio autónomo:
1. Crea una constante llamada PUNTAJE_MINIMO con valor 50.
2. Crea una función llamada es_puntaje_valido que reciba u32 y regrese bool.
3. Crea una función llamada contar_validos que reciba un slice de u32.
4. Usa nombres en snake_case para variables y funciones.
5. Mantén cada función con una responsabilidad clara.
6. Imprime un reporte final desde main.
*/