/*
Sección 5.2 - Métodos y funciones asociadas

Descripción:
Este programa muestra cómo organizar comportamiento dentro de un struct
usando funciones asociadas y métodos definidos en un bloque impl.

Diagrama conceptual:
impl Tipo {
    fn nueva(...) -> Self      -> función asociada
    fn metodo(&self) -> ...    -> método de lectura
    fn cambiar(&mut self)      -> método de modificación
}

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p metodos_funciones_asociadas
*/

struct Rectangulo {
    base: u32,
    altura: u32,
}

impl Rectangulo {
    fn nuevo(base: u32, altura: u32) -> Self {
        Self { base, altura }
    }

    fn area(&self) -> u32 {
        self.base * self.altura
    }

    fn es_cuadrado(&self) -> bool {
        self.base == self.altura
    }

    fn redimensionar(&mut self, nueva_base: u32, nueva_altura: u32) {
        self.base = nueva_base;
        self.altura = nueva_altura;
    }
}

fn main() {
    let mut figura = Rectangulo::nuevo(10, 5);

    println!("Área inicial: {}", figura.area());
    println!("¿Es cuadrado?: {}", figura.es_cuadrado());

    figura.redimensionar(8, 8);

    println!("Área después de redimensionar: {}", figura.area());
    println!("¿Es cuadrado ahora?: {}", figura.es_cuadrado());
}

/*
Ejercicio autónomo:
1. Crea un struct llamado CuentaBancaria con titular y saldo.
2. Implementa una función asociada nueva que construya una cuenta.
3. Implementa un método saldo_actual que use &self y regrese el saldo.
4. Implementa un método depositar que use &mut self y aumente el saldo.
5. Desde main, crea una cuenta, imprime el saldo, deposita una cantidad y vuelve a imprimirlo.
6. Agrega un comentario explicando la diferencia entre una función asociada y un método.
*/