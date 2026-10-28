//  CAMPO	    TIPO	REGRA
//
//  id	        Uuid	Gerado pelo servidor
//  name	    String	Mínimo 3 caracteres
//  email	    String	Obrigatório
//  active	    bool	Inicialmente true

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(transparent)]
pub struct Nome {
    value: String,
}
impl TryFrom<String> for Nome {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().chars().count() < 3 {
            Err("Nome Inválido.".to_owned())
        } else {
            Ok(Self {
                value: value.trim().to_owned(),
            })
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(transparent)]
pub struct Email {
    value: String,
}
impl TryFrom<String> for Email {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            Err("E-mail Inválido".to_owned())
        } else {
            Ok(Self {
                value: value.trim().to_owned(),
            })
        }
    }
}
