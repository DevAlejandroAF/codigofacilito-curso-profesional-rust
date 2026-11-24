/*
Sección 11.4 - Formato y calidad

Descripción:
Esta librería muestra código sencillo para revisar con rustfmt, clippy y cargo fix.
El objetivo es entender cómo las herramientas ayudan a mantener consistencia y calidad.

Diagrama conceptual:
cargo fmt    -> aplica formato consistente
cargo clippy -> revisa lints y posibles mejoras
cargo fix    -> aplica algunas sugerencias automáticas del compilador

Instrucciones:
Ejecuta estos comandos desde la raíz CursoRust:
cargo fmt --package formato_calidad
cargo clippy -p formato_calidad
cargo fix -p formato_calidad --allow-dirty
*/

pub fn calcular_area_circulo(radio: f64) -> f64 {
    let pi = std::f64::consts::PI;
    pi * radio * radio
}

pub fn normalizar_nombre(nombre: &str) -> String {
    nombre.trim().to_lowercase()
}

pub fn esta_activo(valor: bool) -> bool {
    valor
}

pub fn generar_etiquetas(nombres: &[&str]) -> Vec<String> {
    nombres
        .iter()
        .map(|nombre| format!("usuario:{}", normalizar_nombre(nombre)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcula_area_aproximada() {
        let area = calcular_area_circulo(2.0);

        assert!((area - 12.566).abs() < 0.01);
    }

    #[test]
    fn normaliza_nombre_con_espacios() {
        assert_eq!(normalizar_nombre("  ANA  "), "ana");
    }
}

/*
Ejercicio autónomo:
1. Ejecuta cargo fmt --package formato_calidad.
2. Ejecuta cargo clippy -p formato_calidad.
3. Revisa si Clippy propone alguna mejora y aplícala manualmente si tiene sentido.
4. Agrega una función nueva con nombres claros y formato consistente.
5. Ejecuta cargo test -p formato_calidad para verificar que todo siga funcionando.
6. Agrega un comentario explicando qué herramienta usarías para formato y cuál para calidad de código.
*/
