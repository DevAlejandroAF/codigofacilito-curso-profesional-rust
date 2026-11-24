/*
Sección 10.6 - Fundamentos de async/await

Descripción:
Este programa introduce async y await usando Tokio como runtime.
Se simulan operaciones que tardan tiempo para observar tareas asíncronas.

Diagrama conceptual:
async fn consultar(...)
    -> al llamarla produce un Future
    -> con .await esperamos su resultado
    -> el runtime se encarga de avanzar la tarea

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p async_await
*/

use tokio::time::{sleep, Duration};

async fn consultar_servicio(nombre: &str, demora_ms: u64) -> String {
    println!("Iniciando consulta a {nombre}.");

    sleep(Duration::from_millis(demora_ms)).await;

    format!("Respuesta de {nombre} después de {demora_ms} ms")
}

async fn preparar_reporte() -> String {
    let usuarios = consultar_servicio("usuarios", 300).await;
    let cursos = consultar_servicio("cursos", 200).await;

    format!("{usuarios}\n{cursos}")
}

#[tokio::main]
async fn main() {
    let reporte = preparar_reporte().await;
    println!("Reporte secuencial:");
    println!("{reporte}");

    let tarea_cache = tokio::spawn(async {
        consultar_servicio("cache", 150).await
    });

    let respuesta_cache = tarea_cache
        .await
        .expect("La tarea async no debería fallar.");

    println!("Respuesta desde tarea: {respuesta_cache}");
}

/*
Ejercicio autónomo:
1. Crea una función async llamada descargar_dato que reciba un nombre y una demora en milisegundos.
2. Dentro de la función, usa sleep(...).await para simular espera.
3. Haz que la función regrese un String con el nombre del dato.
4. Desde main, llama la función con .await.
5. Después crea una tarea con tokio::spawn y espera su resultado.
6. Agrega un comentario explicando qué papel cumple .await.
*/