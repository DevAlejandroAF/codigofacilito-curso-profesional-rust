use crate::usuarios::Usuario;

pub fn generar_resumen(usuario: &Usuario) -> String {
    format!(
        "Resumen: {} tiene correo {}",
        usuario.nombre(),
        usuario.email_oculto()
    )
}