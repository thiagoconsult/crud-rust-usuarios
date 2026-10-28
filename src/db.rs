// Arc — por que compartilhar ownership?
// Mutex — por que precisamos de exclusividade?
// RwLock — a famosa nuvem do GET.
// State — como o Axum entra nessa história.

use std::sync::{Arc, Mutex};

use crate::usuario::Usuario;

pub struct Database {
    usuarios: Arc<Mutex<Vec<Usuario>>>,
}
impl Database {
    pub fn new() -> Self {
        Self {
            usuarios: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn inserir(&self, usuario: Usuario) -> Result<(), String> {
        let mut guard = self.usuarios.lock().map_err(|e| format!("{e}"))?;
        guard.push(usuario);

        Ok(())
    }
    pub fn listar(&self) -> Result<Vec<Usuario>, String> {
        let guard = self.usuarios.lock().map_err(|e| format!("{e}"))?;
        Ok(guard.clone())
    }
}
impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            usuarios: Arc::clone(&self.usuarios),
        }
    }
}
