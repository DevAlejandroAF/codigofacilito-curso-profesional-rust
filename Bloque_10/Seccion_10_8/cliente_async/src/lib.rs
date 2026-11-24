/*
Sección 10.8 - Ejemplo práctico

Descripción:
Esta librería implementa un cliente asíncrono simulado.
Procesa varias peticiones concurrentemente usando tareas de Tokio.

Diseño general:
1. Modelar una petición con un struct.
2. Modelar una respuesta con un struct.
3. Enviar cada petición como tarea async.
4. Recolectar resultados sin bloquear todo el programa.
5. Mantener main.rs como capa de coordinación.

Ejercicio autónomo:
1. Agrega un campo prioridad: u8 al struct Peticion.
2. Modifica crear_peticiones_demo para asignar prioridades.
3. Crea una función filtrar_prioritarias que reciba un slice de Peticion y regrese solo las de prioridad alta.
4. Procesa solo las peticiones prioritarias desde main.
5. Agrega una función que cuente cuántas respuestas fueron recibidas.
6. Agrega un comentario explicando cómo se combinan concurrencia, async y separación app/librería.
*/

use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
pub struct Peticion {
    pub servicio: String,
    pub demora_ms: u64,
}

#[derive(Debug)]
pub struct Respuesta {
    pub servicio: String,
    pub mensaje: String,
}

pub async fn enviar_peticion(peticion: Peticion) -> Respuesta {
    println!("Enviando petición a {}", peticion.servicio);

    sleep(Duration::from_millis(peticion.demora_ms)).await;

    Respuesta {
        servicio: peticion.servicio,
        mensaje: format!("Respuesta recibida después de {} ms", peticion.demora_ms),
    }
}

pub async fn procesar_peticiones(peticiones: Vec<Peticion>) -> Vec<Respuesta> {
    let mut handles = Vec::new();

    for peticion in peticiones {
        let handle = tokio::spawn(enviar_peticion(peticion));
        handles.push(handle);
    }

    let mut respuestas = Vec::new();

    for handle in handles {
        match handle.await {
            Ok(respuesta) => respuestas.push(respuesta),
            Err(error) => eprintln!("Una tarea falló: {error}"),
        }
    }

    respuestas
}

pub fn crear_peticiones_demo() -> Vec<Peticion> {
    vec![
        Peticion {
            servicio: String::from("usuarios"),
            demora_ms: 400,
        },
        Peticion {
            servicio: String::from("pagos"),
            demora_ms: 250,
        },
        Peticion {
            servicio: String::from("notificaciones"),
            demora_ms: 150,
        },
    ]
}