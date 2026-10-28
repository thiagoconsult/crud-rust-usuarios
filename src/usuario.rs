//  CAMPO	    TIPO	REGRA
//
//  id	        Uuid	Gerado pelo servidor
//  name	    String	Mínimo 3 caracteres
//  email	    String	Obrigatório
//  active	    bool	Inicialmente true

use crate::vos::{Email, Nome};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CriarUsuario {
    pub nome: String,
    pub email: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Usuario {
    id: Uuid,
    nome: Nome,
    email: Email,
    ativo: bool,
}
impl Usuario {
    pub fn new(nome: String, email: String) -> Result<Self, String> {
        let nome_in = Nome::try_from(nome)?;
        let email_in = Email::try_from(email)?;
        let id = Uuid::new_v4();

        Ok(Self {
            id,
            nome: nome_in,
            email: email_in,
            ativo: true,
        })
    }
}
