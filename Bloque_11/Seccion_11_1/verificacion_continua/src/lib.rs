/*
Sección 11.1 - Verificación continua del proyecto

Descripción:
Esta librería pequeña sirve como base para ejecutar comandos de verificación:
cargo check, cargo build, cargo test y cargo doc.

Diagrama conceptual:
cargo check -> revisa errores de compilación sin generar binario final
cargo build -> compila el paquete y genera artefactos
cargo test  -> compila y ejecuta pruebas
cargo doc   -> genera documentación HTML

Instrucciones:
Ejecuta estos comandos desde la raíz CursoRust:
cargo check -p verificacion_continua
cargo build -p verificacion_continua
cargo test -p verificacion_continua
cargo doc -p verificacion_continua --no-deps
*/

/// Calcula el promedio de una lista de calificaciones.
///
/// Regresa `None` cuando la lista está vacía.
pub fn promedio(calificaciones: &[f64]) -> Option<f64> {
    if calificaciones.is_empty() {
        return None;
    }

    let suma: f64 = calificaciones.iter().sum();
    Some(suma / calificaciones.len() as f64)
}

/// Indica si un promedio cumple con el mínimo aprobatorio.
pub fn es_aprobatorio(promedio: f64) -> bool {
    promedio >= 70.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcula_promedio_de_lista_con_datos() {
        let calificaciones = [80.0, 90.0, 100.0];

        assert_eq!(promedio(&calificaciones), Some(90.0));
    }

    #[test]
    fn promedio_de_lista_vacia_es_none() {
        let calificaciones: [f64; 0] = [];

        assert_eq!(promedio(&calificaciones), None);
    }

    #[test]
    fn valida_promedio_aprobatorio() {
        assert!(es_aprobatorio(85.0));
        assert!(!es_aprobatorio(60.0));
    }
}

/*
Ejercicio autónomo:
1. Ejecuta cargo check -p verificacion_continua.
2. Ejecuta cargo build -p verificacion_continua.
3. Ejecuta cargo test -p verificacion_continua.
4. Ejecuta cargo doc -p verificacion_continua --no-deps.
5. Agrega una función llamada calificacion_maxima y una prueba para ella.
6. Agrega un comentario explicando qué comando usarías primero mientras desarrollas y por qué.
*/