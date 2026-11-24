use libreria_documentada::{progreso, resumen, Tarea};

#[test]
fn progreso_con_tareas_mixtas() {
    let mut tarea_1 = Tarea::nueva("Probar integración");
    let tarea_2 = Tarea::nueva("Revisar documentación");

    tarea_1.completar();

    assert_eq!(progreso(&[tarea_1, tarea_2]), Some(50.0));
}

#[test]
fn resumen_con_tareas_muestra_porcentaje() {
    let mut tarea = Tarea::nueva("Terminar módulo");

    tarea.completar();

    assert_eq!(resumen(&[tarea]), "Progreso actual: 100.0%");
}