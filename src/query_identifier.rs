use crate::errors::{ErrorPrograma, ErrorTipo};
use crate::read_file::aplicar_delete;
use crate::read_file::aplicar_insert;
use crate::read_file::aplicar_select;
use crate::read_file::aplicar_update;
use std::error::Error;
use std::iter::Peekable;
use std::str::SplitWhitespace;

#[derive(Debug)]
enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
    Desconocido,
}

//incluí PartialEq únicamente para poder testear el output de las funciones
#[derive(Debug, PartialEq)]
pub struct Insert {
    pub tabla: String,
    pub nombre_col: String,
    pub valores: Vec<String>,
}

#[derive(Debug)]
pub struct Select {
    pub columnas: Vec<String>,
    pub tabla: String,
    pub where_clauses: Option<Vec<WhereClause>>,
    pub order_by: Option<Vec<OrderByClause>>,
    pub operacion_logica: Option<Vec<LogicalOperators>>,
}

#[derive(Debug)]
pub struct Update {
    pub tabla: String,
    pub set: String,
    pub where_clauses: Option<Vec<WhereClause>>,
    pub operacion_logica: Option<Vec<LogicalOperators>>,
}

#[derive(Debug)]
pub struct Delete {
    pub tabla: String,
    pub where_clauses: Option<Vec<WhereClause>>,
    pub operacion_logica: Option<Vec<LogicalOperators>>,
}

#[derive(Debug)]
pub struct WhereClause {
    pub valor1: String,
    pub comparacion: Comparaciones,
    pub valor2: String,
    pub es_not: bool,
}

#[derive(Debug)]
pub struct OrderByClause {
    pub columna: String,
    pub orden: Order,
}

#[derive(Debug)]
pub enum LogicalOperators {
    And,
    Or,
}

#[derive(Debug)]
pub enum Comparaciones {
    Mayor,
    Menor,
    Igual,
    MayorIgual,
    MenorIgual,
    Diferente,
}

#[derive(Debug)]
pub enum Order {
    Asc,
    Desc,
}

type ResultadoClausulas = Result<
    (
        String,
        Vec<WhereClause>,
        Option<Vec<OrderByClause>>,
        Vec<LogicalOperators>,
    ),
    Box<dyn Error>,
>;

impl Insert {
    pub fn new(tabla: &str, columnas: &str, values: Vec<String>) -> Self {
        Insert {
            tabla: tabla.to_string(),
            nombre_col: columnas.to_string(),
            valores: values,
        }
    }

    fn separar_insert_clauses(linea_values: &str) -> Vec<String> {
        let mut valores_a_insertar = Vec::new();
        let caracteres_iter = linea_values.chars().peekable();
        let mut string_actual = String::new();
        let mut dentro_parentesis = false;

        for caracter in caracteres_iter {
            match caracter {
                '(' => {
                    dentro_parentesis = true;
                    string_actual = String::new();
                }
                ')' => {
                    dentro_parentesis = false;
                    valores_a_insertar.push(string_actual.trim().to_string());
                }
                ';' => break,
                _ => {
                    if dentro_parentesis {
                        string_actual.push(caracter);
                    }
                }
            };
        }
        valores_a_insertar
    }

    /*
     * Se recibe un string de la query tipo insert y se parsea para obtener el nombre de la tabla,
     * las columnas y los valores a insertar.
     */
    pub fn insert_parsear_query(query: &str) -> Result<Self, Box<dyn Error>> {
        let posicion_values = query.find("VALUES").ok_or(Box::new(ErrorPrograma::new(
            ErrorTipo::Syntax,
            "No se pudo encontrar VALUES en la query.",
        )))?;

        let linea_tabla = &query[..posicion_values].trim();
        let linea_values = &query[posicion_values + "VALUES".len()..].trim();

        let linea_tabla = &linea_tabla["INSERT INTO ".len()..].trim();
        let pos_columnas_ini = linea_tabla.find("(").ok_or(Box::new(ErrorPrograma::new(
            ErrorTipo::Syntax,
            "Revise si las columnas de la tabla donde insertará se encuentran entre parentesís.",
        )))?;
        let pos_columnas_fin = linea_tabla.find(")").ok_or(Box::new(ErrorPrograma::new(
            ErrorTipo::Syntax,
            "Revise si las columnas de la tabla donde insertará se encuentran entre parentesís.",
        )))?;

        let nombre_tabla = &linea_tabla[..pos_columnas_ini].trim();

        let nombre_columnas = &linea_tabla[1 + pos_columnas_ini..pos_columnas_fin].trim();

        let valores_a_insertar = Self::separar_insert_clauses(linea_values);

        let insert = Insert::new(nombre_tabla, nombre_columnas, valores_a_insertar);
        Ok(insert)
    }
}

fn separar_parentesis(query: &str) -> String {
    let mut query_separada = String::new();
    for caracter in query.chars() {
        if caracter == '(' || caracter == ')' {
            query_separada.push(' ');
            query_separada.push(caracter);
            query_separada.push(' ');
        } else {
            query_separada.push(caracter);
        }
    }
    query_separada
}

impl WhereClause {
    // Crea una Where Clause con los valores ingresados
    pub fn new(valor1: String, comparacion: Comparaciones, valor2: String, es_not: bool) -> Self {
        WhereClause {
            valor1: valor1.to_string(),
            comparacion,
            valor2: valor2.to_string(),
            es_not,
        }
    }
    fn clasificar_valores_clause(
        iter: &mut Peekable<SplitWhitespace>,
    ) -> Result<(String, Comparaciones, String), Box<dyn Error>> {
        let valor1 = iter.next().ok_or(Box::new(ErrorPrograma::new(
            ErrorTipo::Syntax,
            "No se encontró un valor en la cláusula WHERE, recuerde que la clause debe ser \"valor1 comparacion valor2\".",
        )))?;

        let comparacion = match iter.next().ok_or(Box::new(ErrorPrograma::new(
            ErrorTipo::Syntax,
            "No se encontró una comparación en la cláusula WHERE, recuerde que la clause debe ser \"valor1 comparacion valor2\".",
        )))? {
            ">" => Comparaciones::Mayor,
            "<" => Comparaciones::Menor,
            "=" => Comparaciones::Igual,
            ">=" => Comparaciones::MayorIgual,
            "<=" => Comparaciones::MenorIgual,
            "!=" => Comparaciones::Diferente,
            _ => return Err(Box::new(ErrorPrograma::new(
                ErrorTipo::Syntax,
                "El tipo de comparación ingresado no es válido, recuerde que estos pueden ser: =, !=, <, >, <= o >=.",
            ))),
        };

        let valor2 = iter.next().ok_or(Box::new(ErrorPrograma::new(
            ErrorTipo::Syntax,
            "No se encontró un valor en la cláusula WHERE, recuerde que la clause debe ser \"valor1 comparacion valor2\".",
        )))?;

        Ok((valor1.to_string(), comparacion, valor2.to_string()))
    }

    fn clasificar_clauses(
        where_clause: &str,
    ) -> Result<(Vec<WhereClause>, Vec<LogicalOperators>), Box<dyn Error>> {
        let mut where_clauses = Vec::new();
        let mut logical_operators = Vec::new();
        let where_clause_separada = separar_parentesis(where_clause);
        let mut iter = where_clause_separada.split_whitespace().peekable();
        while let Some(&valor) = iter.peek() {
            if valor == "(" || valor == ")" {
                iter.next();
                continue;
            }
            let mut es_not = false;

            if valor == "NOT" {
                es_not = true;
                iter.next();
            }

            let (valor1, comparacion, valor2) = Self::clasificar_valores_clause(&mut iter)?;
            where_clauses.push(WhereClause::new(valor1, comparacion, valor2, es_not));

            if valor == ")" || valor == "(" {
                iter.next();
            }

            if let Some(operador) = iter.next() {
                let operador_logico = match operador {
                    "AND" => LogicalOperators::And,
                    "OR" => LogicalOperators::Or,
                    _ => return Err(Box::new(ErrorPrograma::new(
                        ErrorTipo::Syntax,
                        "El operador lógico ingresado no es válido o la sintaxis fue inválida, los operadores que se pueden manejar son AND, OR y NOT.",
                    ))),
                };
                logical_operators.push(operador_logico);
            }
        }
        Ok((where_clauses, logical_operators))
    }

    /*
     * Se recibe un string que contiene únicamente la where clause y se separa en valores (tipo columna y string) y comparaciones
     * Devuelve la where clause y los operadores lógicos que se encuentren en la query que pueden ser AND u OR.
     */
    pub fn clasificar_where_clause(
        resto: &str,
    ) -> Result<(Vec<WhereClause>, Vec<LogicalOperators>), Box<dyn Error>> {
        let posicion_where = resto.find("WHERE").ok_or(Box::new(ErrorPrograma::new(
            ErrorTipo::Syntax,
            "No se encontro WHERE en la query, revise si esta en minúscula.",
        )))?;
        let posicion_order_by = resto.find("ORDER BY");

        let posicion_where_clause = match posicion_order_by {
            Some(pos) => &resto[posicion_where + "WHERE".len()..pos].trim(),
            None => &resto[posicion_where + "WHERE".len()..].trim(),
        };

        let (where_clauses, logical_operators) = Self::clasificar_clauses(posicion_where_clause)?;

        Ok((where_clauses, logical_operators))
    }
}

impl OrderByClause {
    /*
     * Crea una instancia con los valores ingresados
     */
    pub fn new(columna: &str, orden: Order) -> Self {
        OrderByClause {
            columna: columna.to_string(),
            orden,
        }
    }

    /*
     * Se recibe un string que contiene únicamente de la order by clause y se separa en columnas y
     * orden que puede ser Ascendente o Descendente.
     */
    pub fn clasificar_order_by_clause(resto: &str) -> Result<Vec<OrderByClause>, Box<dyn Error>> {
        let posicion_order_by = resto.find("ORDER BY").ok_or(Box::new(ErrorPrograma::new(
            ErrorTipo::Syntax,
            "No se encontró ORDER BY dentro de la query.",
        )))?;
        let order_by_clause = &resto[posicion_order_by + "ORDER BY".len()..].trim();
        let order_by_clause = order_by_clause.replace(";", "");
        let mut order_by_clauses = Vec::new();
        for i in order_by_clause.split(',') {
            let fraccionado: Vec<&str> = i.split_whitespace().collect();

            let columna = fraccionado.first().ok_or(Box::new(ErrorPrograma::new(
                ErrorTipo::Column,
                "La columna ingresada en el ORDER BY no fue válida o no se encontró.",
            )))?;

            let orden = match fraccionado.get(1) {
                Some(&"ASC") => Order::Asc,
                Some(&"DESC") => Order::Desc,
                Some(_) | None => Order::Asc,
            };

            order_by_clauses.push(OrderByClause::new(columna, orden));
        }

        Ok(order_by_clauses)
    }
}

impl Select {
    /*
     * Crea una instancia del select con los valores ingresados
     */
    pub fn new(
        columnas: Vec<String>,
        tabla: String,
        where_clauses: Option<Vec<WhereClause>>,
        order_by: Option<Vec<OrderByClause>>,
        operacion_logica: Option<Vec<LogicalOperators>>,
    ) -> Self {
        Select {
            columnas,
            tabla: tabla.to_string(),
            where_clauses,
            order_by,
            operacion_logica,
        }
    }

    /*
     * Se recibe un string de la query tipo select y se parsea para obtener las columnas, la tabla, las where clauses y los order by clauses.
     * Se llama a las respectivas funciones que parsean las where y order by clauses a partir de un string.
     */
    pub fn obtener_campos(query: &str, posicion_from: usize) -> ResultadoClausulas {
        let query_limpia = query.trim_end_matches(';');
        let resto = query_limpia[posicion_from..].trim();

        let tabla = match resto["FROM".len()..].split_whitespace().next() {
            Some(tabla) => tabla,
            None => {
                return Err(Box::new(ErrorPrograma::new(
                    ErrorTipo::Syntax,
                    "No se encontro el nombre de la tabla, la sintaxis válidad es: SELECT <columnas> FROM <tabla>.",
                )))
            }
        };
        let (where_clauses, logical_operators) = if resto.contains("WHERE") {
            WhereClause::clasificar_where_clause(resto)?
        } else {
            (Vec::new(), Vec::new())
        };
        let order_by_clauses = if resto.contains("ORDER BY") {
            Some(OrderByClause::clasificar_order_by_clause(resto)?)
        } else {
            None
        };

        Ok((
            tabla.to_string(),
            where_clauses,
            order_by_clauses,
            logical_operators,
        ))
    }

    /*
     * Se parsea la query tipo select a partir del string recibido y se crea una instancia de Select.
     */
    pub fn select_parsear_query(query: &str) -> Result<Self, Box<dyn Error>> {
        let posicion_from = query.find("FROM").ok_or(Box::new(ErrorPrograma::new(
            ErrorTipo::Syntax,
            "No se encontró el FROM dentro de la query, la sintaxis válida es: SELECT <columnas> FROM <tabla>.",
        )))?;
        let columnas: Vec<String> = query["SELECT ".len()..posicion_from]
            .trim()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let (tabla, where_clauses, order_by_clauses, logical_operators) =
            Self::obtener_campos(query, posicion_from)?;

        let select = Select::new(
            columnas,
            tabla,
            Some(where_clauses),
            order_by_clauses,
            Some(logical_operators),
        );
        Ok(select)
    }
}

impl Update {
    /*
     * Se crea una instancia del tipo update con los valores ingresados
     */
    pub fn new(
        tabla: &str,
        set: &str,
        where_clauses: Option<Vec<WhereClause>>,
        operacion_logica: Option<Vec<LogicalOperators>>,
    ) -> Self {
        Update {
            tabla: tabla.to_string(),
            set: set.to_string(),
            where_clauses,
            operacion_logica,
        }
    }

    /*
     * Se parsea la query tipo update a partir del string recibido y se crea una instancia de update.
     */
    pub fn update_parsear_query(query: &str) -> Result<Self, Box<dyn Error>> {
        let posicion_set = query.find("SET").ok_or(Box::new(ErrorPrograma::new(
            ErrorTipo::Syntax,
            "No se encontró el SET dentro de la query, la sintaxis válida es: UPDATE FROM <tabla> WHERE <condiciones>.",
        )))?;
        let linea_tabla = &query[..posicion_set].trim();
        let linea_set_where = &query[posicion_set + "SET".len()..].trim();
        let nombre_tabla = &linea_tabla["UPDATE ".len()..].trim();
        let resto = linea_set_where.trim_end_matches(';');

        let (linea_set, where_clauses, logical_operators) =
            if let Some(posicion_where) = resto.find("WHERE") {
                let linea_set = &resto[..posicion_where].trim();
                let where_clause = &resto[posicion_where..].trim();
                let (where_clauses, logical_operators) =
                    WhereClause::clasificar_where_clause(where_clause)?;
                (
                    linea_set.to_string(),
                    Some(where_clauses),
                    Some(logical_operators),
                )
            } else {
                (resto.to_string(), None, None)
            };

        let update = Update::new(nombre_tabla, &linea_set, where_clauses, logical_operators);
        Ok(update)
    }
}

impl Delete {
    /*
     * Se crea una instancia del tipo delete con los valores ingresados
     */
    pub fn new(
        tabla: &str,
        where_clauses: Option<Vec<WhereClause>>,
        operacion_logica: Option<Vec<LogicalOperators>>,
    ) -> Self {
        Delete {
            tabla: tabla.to_string(),
            where_clauses,
            operacion_logica,
        }
    }

    fn delete_parsear_query(query: &str) -> Result<Self, Box<dyn Error>> {
        let posicion_where = query.find("WHERE").ok_or(Box::new(ErrorPrograma::new(
            ErrorTipo::Syntax,
            "No se encontró WHERE dentro de la query, la sintaxis válida es: DELETE FROM <tabla> WHERE <condición>.",
        )))?;

        let linea_tabla = &query[..posicion_where].trim();
        if !query.starts_with("DELETE FROM") {
            return Err(Box::new(ErrorPrograma::new(
                ErrorTipo::Syntax,
                "No se encontró DELETE FROM dentro de la query, la sintaxis válida es: SELECT <columnas> FROM <tabla>.",
            )));
        }
        let tabla = &linea_tabla["DELETE FROM ".len()..].trim();

        let resto = query[posicion_where..].trim_end_matches(';');

        let (where_clauses, logical_operators) = if resto.contains("WHERE") {
            WhereClause::clasificar_where_clause(resto)?
        } else {
            (Vec::new(), Vec::new())
        };

        let delete = Delete::new(tabla, Some(where_clauses), Some(logical_operators));
        Ok(delete)
    }
}

fn identificar_tipo(query: &str) -> QueryType {
    if query.starts_with("SELECT") {
        QueryType::Select
    } else if query.starts_with("INSERT INTO") {
        QueryType::Insert
    } else if query.starts_with("UPDATE") {
        QueryType::Update
    } else if query.starts_with("DELETE") {
        QueryType::Delete
    } else {
        QueryType::Desconocido
    }
}

/*
 * Dependiendo de la query, se identifica que tipo es, y se llama a la función correspondiente para parsearla, y luego
 * se aplica la query a la tabla correspondiente.
 */
pub fn analisar_query(ruta: &String, query: &str) -> Result<(), Box<dyn Error>> {
    match identificar_tipo(query) {
        QueryType::Insert => {
            let insert = Insert::insert_parsear_query(query)?;
            let ruta_completa = format!("{}/{}.csv", ruta, insert.tabla);
            aplicar_insert(&ruta_completa, &insert.nombre_col, insert.valores)
        }
        QueryType::Select => {
            let select = Select::select_parsear_query(query)?;
            let ruta_completa = format!("{}/{}.csv", ruta, select.tabla);
            aplicar_select(&ruta_completa, &select)
        }
        QueryType::Update => {
            let update = Update::update_parsear_query(query)?;
            let ruta_completa = format!("{}/{}.csv", ruta, update.tabla);
            aplicar_update(&ruta_completa, &update)
        }
        QueryType::Delete => {
            let delete = Delete::delete_parsear_query(query)?;
            let ruta_completa = format!("{}/{}.csv", ruta, delete.tabla);
            aplicar_delete(&ruta_completa, &delete)
        }
        QueryType::Desconocido => Err(Box::new(ErrorPrograma::new(ErrorTipo::Syntax, "La query ingresada es desconocida, no corresponde con SELECT, INSERT, UPDATE o DELETE."))),
    }
}
