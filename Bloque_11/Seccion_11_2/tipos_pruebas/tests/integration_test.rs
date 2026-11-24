use tipos_pruebas::{sumar, titulo_simple};

#[test]
fn suma_desde_prueba_de_integracion() {
    assert_eq!(sumar(10, 15), 25);
}

#[test]
fn titulo_simple_desde_prueba_de_integracion() {
    assert_eq!(titulo_simple("cargo"), "Cargo");
}