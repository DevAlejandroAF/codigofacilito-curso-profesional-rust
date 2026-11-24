use cliente_async::{crear_peticiones_demo, procesar_peticiones};

#[tokio::main]
async fn main() {
    let peticiones = crear_peticiones_demo();

    println!("Procesando {} peticiones.", peticiones.len());

    let respuestas = procesar_peticiones(peticiones).await;

    println!("Respuestas recibidas:");
    for respuesta in respuestas {
        println!("{} -> {}", respuesta.servicio, respuesta.mensaje);
    }
}