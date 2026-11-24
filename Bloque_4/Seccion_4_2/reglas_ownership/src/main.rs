/*
Sección 4.2 - Reglas de ownership

Descripción:
Este programa muestra las reglas principales de ownership: cada valor tiene un owner,
solo puede haber un owner a la vez y el valor se libera cuando su owner sale de scope.

Diagrama conceptual:
1. Cada valor en Rust tiene un owner.
2. Solo puede haber un owner a la vez.
3. Cuando el owner sale de scope, Rust ejecuta drop automáticamente.

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p reglas_ownership
*/

struct Recurso {
    nombre: String,
}

impl Drop for Recurso {
    fn drop(&mut self) {
        println!("Liberando recurso: {}", self.nombre);
    }
}

fn usar_recurso(recurso: Recurso) {
    println!("Usando recurso: {}", recurso.nombre);
}

fn main() {
    let externo = Recurso {
        nombre: String::from("archivo_configuracion"),
    };

    println!("Recurso externo creado: {}", externo.nombre);

    {
        let temporal = Recurso {
            nombre: String::from("conexion_temporal"),
        };

        println!("Recurso temporal creado: {}", temporal.nombre);
    }

    usar_recurso(externo);

    let nuevo_owner = Recurso {
        nombre: String::from("cache_local"),
    };

    let owner_final = nuevo_owner;
    println!("Owner final: {}", owner_final.nombre);
}

/*
Ejercicio autónomo:
1. Crea un struct llamado Documento con un campo titulo de tipo String.
2. Implementa Drop para Documento e imprime un mensaje cuando se libere.
3. Crea un Documento dentro de un bloque interno y observa cuándo se ejecuta drop.
4. Crea otro Documento fuera del bloque y pásalo a una función que tome ownership.
5. Agrega un comentario indicando por qué ya no podrías usar el Documento después de moverlo.
*/