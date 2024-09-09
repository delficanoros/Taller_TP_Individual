use std::env;
use std::error::Error;
mod errors;
mod query_identifier;
mod read_file;
use query_identifier::analisar_query;

struct Comando {
    ruta: String,
    query: String,
}

impl Comando {
    pub fn new(f: &str, q: &str) -> Self {
        Comando {
            ruta: f.to_string(),
            query: q.to_string(),
        }
    }
}

pub fn ejecutar(ruta: &String, query: &str) -> Result<(), Box<dyn Error>> {
    let resultado = analisar_query(ruta, query);

    if let Err(descripcion_error) = resultado {
        println!("{}", descripcion_error);
        return Err(descripcion_error);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let argumentos: Vec<String> = env::args().collect();

    if argumentos.len() < 3 {
        println!(
            "El comando ingresado no es correcto, recuerde que debe tener el siguiente formato:"
        );
        println!("cargo run -- ruta/a/tablas \"<consulta>\"");
        return Ok(());
    }

    let comando = Comando::new(&argumentos[1], &argumentos[2]);
    ejecutar(&comando.ruta, &comando.query)
}
