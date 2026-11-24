/*
Sección 11.3 - Documentación de código

Descripción:
Esta librería muestra comentarios de documentación con /// y //!.
También incluye ejemplos ejecutables que rustdoc puede probar como doc tests.

Diagrama conceptual:
//! -> documenta el crate o módulo completo
/// -> documenta el elemento que viene justo después
# Examples -> sección común para ejemplos ejecutables

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo doc -p documentacion_codigo --no-deps --open
cargo test -p documentacion_codigo --doc
*/

//! Utilidades pequeñas para generar mensajes académicos.
//!
//! Este crate muestra cómo escribir documentación útil para funciones públicas.

/// Representa un curso con nombre y número de horas.
#[derive(Debug, Clone, PartialEq)]
pub struct Curso {
    /// Nombre visible del curso.
    pub nombre: String,
    /// Duración total del curso en horas.
    pub horas: u32,
}

impl Curso {
    /// Crea un nuevo curso.
    ///
    /// # Examples
    ///
    /// ```
    /// use documentacion_codigo::Curso;
    ///
    /// let curso = Curso::nuevo("Rust", 20);
    ///
    /// assert_eq!(curso.nombre, "Rust");
    /// assert_eq!(curso.horas, 20);
    /// ```
    pub fn nuevo(nombre: &str, horas: u32) -> Self {
        Self {
            nombre: nombre.to_string(),
            horas,
        }
    }
}

/// Genera una descripción legible para un curso.
///
/// # Examples
///
/// ```
/// use documentacion_codigo::{describir_curso, Curso};
///
/// let curso = Curso::nuevo("Testing", 8);
///
/// assert_eq!(describir_curso(&curso), "Testing tiene una duración de 8 horas.");
/// ```
pub fn describir_curso(curso: &Curso) -> String {
    format!("{} tiene una duración de {} horas.", curso.nombre, curso.horas)
}

/*
Ejercicio autónomo:
1. Agrega un campo nivel: String al struct Curso.
2. Actualiza la documentación de Curso para explicar ese campo.
3. Actualiza el constructor nuevo para recibir nivel.
4. Agrega un ejemplo ejecutable en la documentación del constructor.
5. Ejecuta cargo test -p documentacion_codigo --doc.
6. Agrega un comentario explicando por qué los ejemplos en documentación deben mantenerse actualizados.
*/