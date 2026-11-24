/*
Sección 11.5 - Ejemplo práctico

Descripción:
Esta librería pequeña integra documentación, pruebas unitarias, pruebas de integración
y ejemplos ejecutables en comentarios de documentación.

Objetivo del programa:
1. Modelar tareas de estudio.
2. Calcular progreso.
3. Generar mensajes de resumen.
4. Documentar la API pública.
5. Probar comportamiento normal y casos borde.
*/

//! Librería pequeña para gestionar tareas de estudio.
//!
//! Este crate se usa para practicar documentación y pruebas en Rust.

/// Representa una tarea de estudio.
#[derive(Debug, Clone, PartialEq)]
pub struct Tarea {
    /// Descripción visible de la tarea.
    pub descripcion: String,
    /// Indica si la tarea ya fue completada.
    pub completada: bool,
}

impl Tarea {
    /// Crea una tarea nueva como pendiente.
    ///
    /// # Examples
    ///
    /// ```
    /// use libreria_documentada::Tarea;
    ///
    /// let tarea = Tarea::nueva("Repasar pruebas");
    ///
    /// assert_eq!(tarea.descripcion, "Repasar pruebas");
    /// assert!(!tarea.completada);
    /// ```
    pub fn nueva(descripcion: &str) -> Self {
        Self {
            descripcion: descripcion.to_string(),
            completada: false,
        }
    }

    /// Marca la tarea como completada.
    pub fn completar(&mut self) {
        self.completada = true;
    }
}

/// Calcula el porcentaje de tareas completadas.
///
/// Regresa `None` si la lista está vacía.
///
/// # Examples
///
/// ```
/// use libreria_documentada::{progreso, Tarea};
///
/// let mut tarea = Tarea::nueva("Documentar");
/// tarea.completar();
///
/// assert_eq!(progreso(&[tarea]), Some(100.0));
/// ```
pub fn progreso(tareas: &[Tarea]) -> Option<f64> {
    if tareas.is_empty() {
        return None;
    }

    let completadas = tareas.iter().filter(|tarea| tarea.completada).count();

    Some(completadas as f64 * 100.0 / tareas.len() as f64)
}

/// Genera un resumen breve del progreso.
pub fn resumen(tareas: &[Tarea]) -> String {
    match progreso(tareas) {
        Some(porcentaje) => format!("Progreso actual: {porcentaje:.1}%"),
        None => String::from("No hay tareas registradas."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tarea_nueva_inicia_pendiente() {
        let tarea = Tarea::nueva("Leer documentación");

        assert!(!tarea.completada);
    }

    #[test]
    fn completar_cambia_estado() {
        let mut tarea = Tarea::nueva("Escribir pruebas");

        tarea.completar();

        assert!(tarea.completada);
    }

    #[test]
    fn progreso_de_lista_vacia_es_none() {
        let tareas: Vec<Tarea> = Vec::new();

        assert_eq!(progreso(&tareas), None);
    }

    #[test]
    fn resumen_sin_tareas_es_claro() {
        assert_eq!(resumen(&[]), "No hay tareas registradas.");
    }
}

/*
Ejercicio autónomo:
1. Agrega un campo prioridad: u8 a Tarea.
2. Actualiza Tarea::nueva para recibir prioridad.
3. Documenta el nuevo campo y actualiza los ejemplos.
4. Agrega una función tareas_pendientes que regrese cuántas tareas no están completadas.
5. Escribe pruebas unitarias y una prueba de integración para tareas_pendientes.
6. Ejecuta cargo test, cargo doc y cargo clippy sobre este paquete.
*/