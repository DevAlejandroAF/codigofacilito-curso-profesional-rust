pub struct Usuario {
    nombre: String,
    email: String,
}

impl Usuario {
    pub fn nuevo(nombre: &str, email: &str) -> Self {
        Self {
            nombre: String::from(nombre),
            email: String::from(email),
        }
    }

    pub fn nombre(&self) -> &str {
        &self.nombre
    }

    pub fn email_oculto(&self) -> String {
        let dominio = self.email.split('@').nth(1).unwrap_or("desconocido");
        format!("***@{dominio}")
    }
}