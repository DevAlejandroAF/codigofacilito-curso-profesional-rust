/*
Sección 6.2 - Traits

Descripción:
Este programa muestra cómo definir un trait para comportamiento compartido
y cómo implementarlo en tipos propios.

Diagrama conceptual:
trait Describible {
    fn describir(&self) -> String;
}

impl Describible for Tipo {
    fn describir(&self) -> String {
        ...
    }
}

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p traits
*/

trait Describible {
    fn describir(&self) -> String;
}

struct Curso {
    nombre: String,
    duracion_horas: u32,
}

struct Mentor {
    nombre: String,
    especialidad: String,
}

impl Describible for Curso {
    fn describir(&self) -> String {
        format!("Curso: {}. Duración: {} horas.", self.nombre, self.duracion_horas)
    }
}

impl Describible for Mentor {
    fn describir(&self) -> String {
        format!("Mentor: {}. Especialidad: {}.", self.nombre, self.especialidad)
    }
}

fn imprimir_descripcion(item: &impl Describible) {
    println!("{}", item.describir());
}

fn main() {
    let curso = Curso {
        nombre: String::from("Rust desde cero"),
        duracion_horas: 24,
    };

    let mentor = Mentor {
        nombre: String::from("Ana"),
        especialidad: String::from("programación de sistemas"),
    };

    imprimir_descripcion(&curso);
    imprimir_descripcion(&mentor);
}

/*
Ejercicio autónomo:
1. Crea un trait llamado Notificable con un método notificar que regrese String.
2. Crea un struct Email con destinatario y asunto.
3. Crea un struct AlertaSistema con nivel y mensaje.
4. Implementa Notificable para ambos structs.
5. Crea una función que reciba &impl Notificable e imprima la notificación.
6. Prueba la función desde main con una instancia de cada tipo.
*/