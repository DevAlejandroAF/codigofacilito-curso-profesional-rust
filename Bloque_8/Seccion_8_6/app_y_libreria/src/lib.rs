/*
Sección 8.6 - Separación entre app y librería

Descripción:
Este archivo contiene lógica de negocio reutilizable y testeable.
La app ejecutable solo debe coordinar entrada, salida y llamadas a esta librería.

Diagrama conceptual:
lib.rs  -> reglas, cálculos y funciones reutilizables
main.rs -> punto de entrada, impresión y coordinación del flujo
*/

pub fn calcular_promedio(calificaciones: &[f64]) -> Option<f64> {
    if calificaciones.is_empty() {
        return None;
    }

    let suma: f64 = calificaciones.iter().sum();
    Some(suma / calificaciones.len() as f64)
}

pub fn estado_academico(promedio: f64) -> &'static str {
    if promedio >= 90.0 {
        "sobresaliente"
    } else if promedio >= 70.0 {
        "aprobado"
    } else {
        "no aprobado"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcula_promedio_de_lista() {
        let calificaciones = [80.0, 90.0, 100.0];
        assert_eq!(calcular_promedio(&calificaciones), Some(90.0));
    }

    #[test]
    fn promedio_de_lista_vacia_es_none() {
        let calificaciones: [f64; 0] = [];
        assert_eq!(calcular_promedio(&calificaciones), None);
    }
}

/*
Ejercicio autónomo:
1. Agrega una función pública llamada es_aprobatorio.
2. La función debe recibir un promedio f64 y regresar bool.
3. Usa esa función dentro de estado_academico si lo consideras útil.
4. Agrega una prueba unitaria para es_aprobatorio.
5. Usa la función desde main.rs.
6. Agrega un comentario explicando por qué lib.rs es más fácil de probar que main.rs.
*/