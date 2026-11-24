/*
Sección 10.7 - Introducción a runtimes async

Descripción:
Este programa muestra cómo Tokio ejecuta tareas asíncronas concurrentes.
Se lanzan varias tareas con tokio::spawn y se esperan sus resultados.

Diagrama conceptual:
#[tokio::main]
async fn main()
    ├── spawn tarea usuarios
    ├── spawn tarea pagos
    └── spawn tarea inventario

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p tokio_runtime
*/

use tokio::time::{sleep, Duration, Instant};

async fn consultar(nombre: &'static str, demora_ms: u64) -> String {
    println!("Iniciando tarea {nombre}");

    sleep(Duration::from_millis(demora_ms)).await;

    format!("{nombre} terminó en {demora_ms} ms")
}

#[tokio::main]
async fn main() {
    let inicio = Instant::now();

    let usuarios = tokio::spawn(consultar("usuarios", 500));
    let pagos = tokio::spawn(consultar("pagos", 300));
    let inventario = tokio::spawn(consultar("inventario", 100));

    let resultado_usuarios = usuarios.await.expect("La tarea usuarios no debería fallar.");
    let resultado_pagos = pagos.await.expect("La tarea pagos no debería fallar.");
    let resultado_inventario = inventario
        .await
        .expect("La tarea inventario no debería fallar.");

    println!("{resultado_usuarios}");
    println!("{resultado_pagos}");
    println!("{resultado_inventario}");
    println!("Tiempo total aproximado: {:?}", inicio.elapsed());
}

/*
Ejercicio autónomo:
1. Crea una función async llamada procesar_servicio que reciba un nombre y una demora.
2. Lanza cuatro tareas con tokio::spawn.
3. Usa .await sobre cada JoinHandle para recuperar los resultados.
4. Imprime los resultados y el tiempo total aproximado.
5. Cambia las demoras y observa cómo afecta el orden de inicio y finalización.
6. Agrega un comentario explicando qué responsabilidad tiene el runtime async.
*/