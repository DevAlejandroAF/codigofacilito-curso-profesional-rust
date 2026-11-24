/*
Sección 12.4 - Panorama de interoperabilidad

Descripción:
Este programa introduce ideas básicas de interoperabilidad:
representación compatible con C, funciones extern "C" y conversión cuidadosa de datos.

Diagrama conceptual:
Rust seguro
    -> prepara datos con representación estable
    -> llama o expone una frontera extern "C"
    -> valida entradas y salidas al volver a Rust seguro

Idea clave:
La frontera FFI debe ser pequeña, clara y documentada.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p interoperabilidad
*/

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PuntoC {
    x: i32,
    y: i32,
}

pub extern "C" fn distancia_manhattan(punto: PuntoC) -> i32 {
    punto.x.abs() + punto.y.abs()
}

fn crear_punto_seguro(x: i32, y: i32) -> PuntoC {
    PuntoC { x, y }
}

fn procesar_punto(punto: PuntoC) -> String {
    let distancia = distancia_manhattan(punto);
    format!("Punto ({}, {}) con distancia Manhattan {}", punto.x, punto.y, distancia)
}

fn main() {
    let punto = crear_punto_seguro(-3, 7);
    let resumen = procesar_punto(punto);

    println!("{resumen}");
}

/*
Ejercicio autónomo:
1. Crea un struct RectanguloC con #[repr(C)] y campos ancho: i32 y alto: i32.
2. Crea una función pub extern "C" llamada area_rectangulo.
3. La función debe recibir RectanguloC y regresar i32.
4. Crea una función segura que valide que ancho y alto sean positivos antes de llamar la función extern.
5. Prueba el caso válido y el caso inválido desde main.
6. Agrega un comentario explicando por qué conviene validar datos antes o después de una frontera FFI.
*/