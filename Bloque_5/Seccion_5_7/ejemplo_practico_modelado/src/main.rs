/*
Sección 5.7 - Ejemplo práctico

Descripción:
Este programa modela un flujo sencillo de una solicitud usando structs,
enums, Option<T> y match para representar estados y decisiones del sistema.

Objetivo del programa:
1. Crear una solicitud con datos básicos.
2. Representar su estado con un enum.
3. Procesar el estado con match.
4. Usar Option para modelar un aprobador opcional.
5. Mostrar un resumen final del flujo.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p ejemplo_practico_modelado
*/

struct Solicitud {
    id: u32,
    solicitante: String,
    aprobador: Option<String>,
    estado: EstadoSolicitud,
}

enum EstadoSolicitud {
    EnRevision,
    Aprobada(String),
    Rechazada(String),
}

fn describir_solicitud(solicitud: &Solicitud) {
    println!("Solicitud #{}", solicitud.id);
    println!("Solicitante: {}", solicitud.solicitante);

    match &solicitud.aprobador {
        Some(nombre) => println!("Aprobador asignado: {nombre}"),
        None => println!("Aún no hay aprobador asignado."),
    }

    match &solicitud.estado {
        EstadoSolicitud::EnRevision => {
            println!("Estado actual: en revisión.");
        }
        EstadoSolicitud::Aprobada(comentario) => {
            println!("Estado actual: aprobada.");
            println!("Comentario: {comentario}");
        }
        EstadoSolicitud::Rechazada(motivo) => {
            println!("Estado actual: rechazada.");
            println!("Motivo: {motivo}");
        }
    }
}

fn main() {
    let solicitud_1 = Solicitud {
        id: 101,
        solicitante: String::from("Maeva"),
        aprobador: Some(String::from("Laura")),
        estado: EstadoSolicitud::Aprobada(String::from("Cumple todos los requisitos.")),
    };

    let solicitud_2 = Solicitud {
        id: 102,
        solicitante: String::from("Luis"),
        aprobador: None,
        estado: EstadoSolicitud::EnRevision,
    };

    describir_solicitud(&solicitud_1);
    println!("--------------------");
    describir_solicitud(&solicitud_2);
}

/*
Ejercicio autónomo:
1. Agrega una tercera solicitud con estado Rechazada y un motivo.
2. Crea una función llamada esta_finalizada que reciba &Solicitud y regrese bool.
3. Haz que regrese true para Aprobada o Rechazada, y false para EnRevision.
4. Llama esa función desde main para cada solicitud.
5. Imprime si cada solicitud está finalizada o no.
6. Agrega un comentario explicando cómo structs y enums se complementan para modelar estados.
*/