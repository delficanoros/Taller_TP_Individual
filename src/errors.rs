use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ErrorTipo {
    Syntax,
    Table,
    Column,
}

#[derive(Debug)]
pub struct ErrorPrograma {
    tipo: ErrorTipo,
    descripcion: String,
}

impl ErrorPrograma {
    pub fn new(tipo: ErrorTipo, descripcion: &str) -> Self {
        ErrorPrograma {
            tipo,
            descripcion: descripcion.to_string(),
        }
    }
}

impl Error for ErrorPrograma {}

impl fmt::Display for ErrorPrograma {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid {:?}: {}", self.tipo, self.descripcion)
    }
}
