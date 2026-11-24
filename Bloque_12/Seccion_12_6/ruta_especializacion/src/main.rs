/*
Sección 12.6 - Ruta de especialización posterior al curso

Descripción:
Este programa propone rutas de especialización después del curso.
Usa enums, structs y pattern matching para recomendar siguientes pasos según perfil.

Diagrama conceptual:
Perfil del estudiante
    -> intereses técnicos
    -> ruta sugerida
    -> práctica recomendada

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p ruta_especializacion
*/

#[derive(Debug)]
enum Perfil {
    Backend,
    Sistemas,
    Embedded,
    WebAssembly,
    HerramientasCli,
}

struct Ruta {
    nombre: &'static str,
    temas: &'static [&'static str],
    proyecto: &'static str,
}

fn ruta_para(perfil: Perfil) -> Ruta {
    match perfil {
        Perfil::Backend => Ruta {
            nombre: "Rust para backend",
            temas: &["Tokio", "APIs HTTP", "bases de datos", "observabilidad"],
            proyecto: "Construir una API con autenticación y persistencia",
        },
        Perfil::Sistemas => Ruta {
            nombre: "Rust de sistemas",
            temas: &["memoria", "unsafe controlado", "FFI", "profiling"],
            proyecto: "Crear una librería que procese archivos binarios",
        },
        Perfil::Embedded => Ruta {
            nombre: "Rust embedded",
            temas: &["no_std", "periféricos", "HALs", "tiempo real"],
            proyecto: "Leer sensores y reportar mediciones",
        },
        Perfil::WebAssembly => Ruta {
            nombre: "Rust y WebAssembly",
            temas: &["wasm-bindgen", "wasm-pack", "interoperabilidad con JS", "tamaño de binario"],
            proyecto: "Crear un módulo Wasm para procesamiento de datos en navegador",
        },
        Perfil::HerramientasCli => Ruta {
            nombre: "Herramientas CLI profesionales",
            temas: &["clap", "configuración", "testing", "distribución"],
            proyecto: "Crear una utilidad CLI con subcomandos y archivos de configuración",
        },
    }
}

fn imprimir_ruta(perfil: Perfil) {
    let ruta = ruta_para(perfil);

    println!("Ruta sugerida: {}", ruta.nombre);
    println!("Temas a estudiar: {}", ruta.temas.join(", "));
    println!("Proyecto recomendado: {}", ruta.proyecto);
}

fn main() {
    imprimir_ruta(Perfil::Backend);
    println!("---");
    imprimir_ruta(Perfil::Sistemas);
    println!("---");
    imprimir_ruta(Perfil::Embedded);
    println!("---");
    imprimir_ruta(Perfil::WebAssembly);
    println!("---");
    imprimir_ruta(Perfil::HerramientasCli);
}

/*
Ejercicio autónomo:
1. Agrega un perfil llamado Seguridad.
2. Define una ruta con temas como análisis de binarios, fuzzing y auditoría de dependencias.
3. Agrega un proyecto recomendado para ese perfil.
4. Imprime la ruta desde main.
5. Crea una función que reciba una ruta y regrese el primer tema recomendado.
6. Agrega un comentario explicando cuál ruta elegirías después del curso y por qué.
*/