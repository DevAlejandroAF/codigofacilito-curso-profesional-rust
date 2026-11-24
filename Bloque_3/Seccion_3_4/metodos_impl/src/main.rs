/*
Sección 3.4 - Métodos e implementación con impl

Descripción:
Este programa muestra cómo definir métodos dentro de un bloque impl.
Se usan métodos con self, &self y &mut self, además de una función asociada.

Diagrama conceptual:
impl Tipo {
    fn nueva(...) -> Self      -> función asociada, no recibe self
    fn leer(&self) -> ...      -> método que lee la instancia
    fn cambiar(&mut self)      -> método que modifica la instancia
    fn consumir(self) -> ...   -> método que toma ownership de la instancia
}

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p metodos_impl
*/

struct Cuenta {
    titular: String,
    saldo: f64,
}

impl Cuenta {
    fn nueva(titular: &str, saldo_inicial: f64) -> Self {
        Self {
            titular: String::from(titular),
            saldo: saldo_inicial,
        }
    }

    fn resumen(&self) -> String {
        format!("{} tiene un saldo de ${:.2}", self.titular, self.saldo)
    }

    fn depositar(&mut self, cantidad: f64) {
        self.saldo += cantidad;
    }

    fn cerrar(self) -> String {
        format!("Cuenta de {} cerrada con ${:.2}", self.titular, self.saldo)
    }
}

fn main() {
    let mut cuenta = Cuenta::nueva("Ana", 1500.0);

    println!("{}", cuenta.resumen());

    cuenta.depositar(500.0);
    println!("{}", cuenta.resumen());

    let mensaje_cierre = cuenta.cerrar();
    println!("{mensaje_cierre}");
}

/*
Ejercicio autónomo:
1. Crea un struct llamado Curso con nombre, estudiantes_inscritos y cupo_maximo.
2. Implementa una función asociada nueva para crear un Curso.
3. Implementa un método hay_cupo que use &self y regrese bool.
4. Implementa un método inscribir_estudiante que use &mut self y aumente estudiantes_inscritos solo si hay cupo.
5. Desde main, crea un curso, intenta inscribir estudiantes e imprime el resultado.
*/