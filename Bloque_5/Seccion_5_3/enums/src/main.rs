/*
Sección 5.3 - enums

Descripción:
Este programa introduce enums con variantes simples y variantes
que contienen datos asociados.

Diagrama conceptual:
enum Nombre {
    VarianteSimple,
    VarianteConDato(Tipo),
    VarianteConCampos { campo: Tipo },
}

Instrucciones:
Ejecuta este paquete desde la raíz CursoRust con:
cargo run -p enums
*/

enum EstadoPedido {
    Pendiente,
    EnCamino(String),
    Entregado { fecha: String, recibido_por: String },
}

fn describir_estado(estado: &EstadoPedido) {
    match estado {
        EstadoPedido::Pendiente => {
            println!("El pedido todavía está pendiente.");
        }
        EstadoPedido::EnCamino(destino) => {
            println!("El pedido va en camino a {destino}.");
        }
        EstadoPedido::Entregado { fecha, recibido_por } => {
            println!("El pedido fue entregado el {fecha} y lo recibió {recibido_por}.");
        }
    }
}

fn main() {
    let pedido_1 = EstadoPedido::Pendiente;
    let pedido_2 = EstadoPedido::EnCamino(String::from("Puebla"));
    let pedido_3 = EstadoPedido::Entregado {
        fecha: String::from("2026-05-24"),
        recibido_por: String::from("Ana"),
    };

    describir_estado(&pedido_1);
    describir_estado(&pedido_2);
    describir_estado(&pedido_3);
}

/*
Ejercicio autónomo:
1. Crea un enum llamado MetodoPago.
2. Agrega una variante simple llamada Efectivo.
3. Agrega una variante con dato asociado llamada Transferencia(String).
4. Agrega una variante con campos nombrados llamada Tarjeta { banco: String, ultimos_digitos: u16 }.
5. Crea una función que reciba una referencia al enum y describa el método de pago con match.
6. Prueba las tres variantes desde main.
*/