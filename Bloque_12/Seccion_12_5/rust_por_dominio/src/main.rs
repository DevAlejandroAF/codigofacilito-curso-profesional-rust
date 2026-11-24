/*
Sección 12.5 - Rust según dominio de aplicación

Descripción:
Este programa modela diferencias entre dominios donde Rust se usa con frecuencia:
backend, CLI, embedded y WebAssembly. La idea es comparar prioridades técnicas,
no construir una aplicación completa para cada dominio.

Diagrama conceptual:
Backend     -> servicios, concurrencia, APIs, persistencia
CLI         -> argumentos, archivos, experiencia de terminal
Embedded    -> recursos limitados, hardware, no_std en algunos casos
WebAssembly -> integración con web, tamaño del binario, frontera JS/Wasm

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p rust_por_dominio
*/

#[derive(Debug)]
enum Dominio {
    Backend,
    Cli,
    Embedded,
    WebAssembly,
}

struct Recomendacion {
    enfoque: &'static str,
    herramientas: &'static [&'static str],
    cuidado_principal: &'static str,
}

fn recomendar(dominio: Dominio) -> Recomendacion {
    match dominio {
        Dominio::Backend => Recomendacion {
            enfoque: "servicios confiables y procesamiento concurrente",
            herramientas: &["axum", "tokio", "serde", "sqlx"],
            cuidado_principal: "manejo de errores, observabilidad y límites de concurrencia",
        },
        Dominio::Cli => Recomendacion {
            enfoque: "herramientas rápidas y agradables de usar en terminal",
            herramientas: &["clap", "std::fs", "std::env", "anyhow"],
            cuidado_principal: "mensajes de ayuda claros y errores accionables",
        },
        Dominio::Embedded => Recomendacion {
            enfoque: "programas cercanos al hardware y con recursos limitados",
            herramientas: &["embedded-hal", "probe-rs", "defmt", "no_std"],
            cuidado_principal: "memoria limitada, consumo de energía y acceso a periféricos",
        },
        Dominio::WebAssembly => Recomendacion {
            enfoque: "módulos compilados para ejecutarse desde entornos web",
            herramientas: &["wasm-bindgen", "wasm-pack", "web-sys", "js-sys"],
            cuidado_principal: "tamaño del binario y frontera entre Rust y JavaScript",
        },
    }
}

fn imprimir_recomendacion(dominio: Dominio) {
    let recomendacion = recomendar(dominio);

    println!("Enfoque: {}", recomendacion.enfoque);
    println!("Herramientas comunes: {}", recomendacion.herramientas.join(", "));
    println!("Cuidado principal: {}", recomendacion.cuidado_principal);
}

fn main() {
    imprimir_recomendacion(Dominio::Backend);
    println!("---");
    imprimir_recomendacion(Dominio::Cli);
    println!("---");
    imprimir_recomendacion(Dominio::Embedded);
    println!("---");
    imprimir_recomendacion(Dominio::WebAssembly);
}

/*
Ejercicio autónomo:
1. Agrega una variante nueva al enum Dominio llamada CienciaDeDatos.
2. Agrega su recomendación en la función recomendar.
3. Incluye al menos tres herramientas o crates posibles.
4. Imprime la recomendación desde main.
5. Agrega una función que regrese solo el cuidado principal de un dominio.
6. Agrega un comentario explicando por qué el estilo de Rust cambia según el dominio.
*/