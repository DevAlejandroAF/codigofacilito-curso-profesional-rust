/*
Sección 11.2 - Pruebas unitarias, de integración y doc tests

Descripción:
Esta librería muestra tres tipos de pruebas:
unitarias dentro del módulo, de integración en tests/ y doc tests en comentarios.

Diagrama conceptual:
src/lib.rs
├── funciones públicas
├── #[cfg(test)] mod tests -> pruebas unitarias
└── /// ``` ... ```       -> doc tests

tests/integration_test.rs -> pruebas de integración como usuario externo del crate
*/

/// Suma dos números enteros.
///
/// # Examples
///
/// ```
/// use tipos_pruebas::sumar;
///
/// assert_eq!(sumar(2, 3), 5);
/// ```
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

/// Convierte un texto a formato de título simple.
///
/// # Examples
///
/// ```
/// use tipos_pruebas::titulo_simple;
///
/// assert_eq!(titulo_simple("rust"), "Rust");
/// ```
pub fn titulo_simple(texto: &str) -> String {
    let mut caracteres = texto.chars();

    match caracteres.next() {
        Some(primero) => primero.to_uppercase().collect::<String>() + caracteres.as_str(),
        None => String::new(),
    }
}

fn normalizar_espacios(texto: &str) -> String {
    texto.split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sumar_funciona_con_positivos() {
        assert_eq!(sumar(4, 6), 10);
    }

    #[test]
    fn titulo_simple_con_texto_vacio() {
        assert_eq!(titulo_simple(""), "");
    }

    #[test]
    fn normaliza_espacios_internamente() {
        assert_eq!(normalizar_espacios("Rust   con   pruebas"), "Rust con pruebas");
    }
}

/*
Ejercicio autónomo:
1. Agrega una función pública llamada multiplicar.
2. Escribe una prueba unitaria para multiplicar dentro de src/lib.rs.
3. Agrega un doc test en el comentario de documentación de multiplicar.
4. Escribe una prueba de integración en tests/integration_test.rs.
5. Ejecuta cargo test -p tipos_pruebas.
6. Agrega un comentario explicando qué diferencia hay entre prueba unitaria, integración y doc test.
*/