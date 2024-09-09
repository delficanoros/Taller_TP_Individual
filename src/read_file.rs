use crate::errors::{ErrorPrograma, ErrorTipo};
use crate::query_identifier::{
    Comparaciones, Delete, LogicalOperators, Order, Select, Update, WhereClause,
};
use std::error::Error;
use std::fs::rename;
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::io::Write;
use std::io::{BufRead, BufReader, Lines};

type ResultLeerArchivo = Result<(Lines<BufReader<File>>, Vec<String>), Box<dyn Error>>;

fn abrir_archivo(ruta: &String) -> ResultLeerArchivo {
    let file = File::open(ruta)?;
    let reader = BufReader::new(file);
    let mut lineas = reader.lines();

    let columnas_csv = lineas.next().ok_or(Box::new(ErrorPrograma::new(
        ErrorTipo::Table,
        "El archivo csv se encuentra vacío",
    )))??;

    let columnas_csv: Vec<String> = columnas_csv
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    Ok((lineas, columnas_csv))
}

fn es_int(valor: &str) -> bool {
    valor.parse::<i32>().is_ok()
}

fn verificar_existen_columnas_query(
    columnas_query: &[String],
    columnas_csv: &[String],
) -> Result<(), Box<dyn Error>> {
    for columna in columnas_query.iter() {
        if !columnas_csv.contains(columna) {
            let descripcion = format!(
                "La columna {} no fue encontrada en el archivo csv.",
                columna
            );
            return Err(Box::new(ErrorPrograma::new(
                ErrorTipo::Column,
                descripcion.as_str(),
            )));
        }
    }

    Ok(())
}

fn verificar_existen_columnas_where_clause(
    where_clauses: &Option<Vec<WhereClause>>,
    columnas_csv: &[String],
) -> Result<(), Box<dyn Error>> {
    if let Some(ref clauses) = where_clauses {
        for clause in clauses.iter() {
            if columnas_csv.contains(&clause.valor1)
                && (clause.valor2.contains('\'') || es_int(&clause.valor2))
            {
                continue;
            };
            if columnas_csv.contains(&clause.valor2)
                && (clause.valor1.contains('\'') || es_int(&clause.valor1))
            {
                continue;
            }
            if columnas_csv.contains(&clause.valor2) && columnas_csv.contains(&clause.valor1) {
                continue;
            }
            let descripcion = format!("Ninguno de estos dos valores '{}' y '{}' en la WHERE clause fue encontrado como una columna en el csv.", clause.valor1, clause.valor2);
            return Err(Box::new(ErrorPrograma::new(
                ErrorTipo::Column,
                &descripcion,
            )));
        }
    };
    Ok(())
}

fn filtrar_lineas_que_cumplen_con_where_clause(
    lineas: Lines<BufReader<File>>,
    select: &Select,
    columnas_csv: &[String],
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut lineas_filtradas: Vec<Vec<String>> = vec![];

    for linea in lineas {
        let linea = linea?;
        let valores: Vec<String> = linea.split(',').map(|s| s.trim().to_string()).collect();

        if let Some(ref condiciones) = select.where_clauses {
            if !condiciones.is_empty() {
                if let Some(ref operadores_logicos) = select.operacion_logica {
                    if !aplicar_filtro(&valores, columnas_csv, condiciones, operadores_logicos)? {
                        continue;
                    }
                }
            }
        }

        lineas_filtradas.push(valores.iter().map(|s| s.to_string()).collect());
    }
    Ok(lineas_filtradas)
}

fn ordenar_lineas_select(
    select: &Select,
    mut lineas_filtradas: Vec<Vec<String>>,
    columnas_csv: &[String],
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    if let Some(ref ordernar) = select.order_by {
        for ordenamiento_actual in ordernar.iter() {
            if !columnas_csv.contains(&ordenamiento_actual.columna) {
                let descripcion = format!(
                    "La columna '{}' ingresada en ORDER BY clause no se encuentra en el csv.",
                    ordenamiento_actual.columna
                );
                return Err(Box::new(ErrorPrograma::new(
                    ErrorTipo::Column,
                    &descripcion,
                )));
            }
        }
        lineas_filtradas.sort_by(|a, b| {
            for ordenamiento_actual in ordernar.iter() {
                let indice = columnas_csv
                    .iter()
                    .position(|col| *col == ordenamiento_actual.columna)
                    .unwrap_or(0);
                let resultado_comparado = match ordenamiento_actual.orden {
                    Order::Asc => a[indice].cmp(&b[indice]),
                    Order::Desc => b[indice].cmp(&a[indice]),
                };

                if resultado_comparado != std::cmp::Ordering::Equal {
                    return resultado_comparado;
                }
            }
            std::cmp::Ordering::Equal
        });
    }
    Ok(lineas_filtradas)
}

fn seleccionar_y_escribir_columnas_pedidas(
    lineas_filtradas: Vec<Vec<String>>,
    select: &Select,
    columnas_csv: &[String],
) -> Result<(), Box<dyn Error>> {
    for linea in lineas_filtradas.iter() {
        if select.columnas[0] == "*" {
            println!("{}", linea.join(","));
        } else {
            let linea: Vec<&str> = select
                .columnas
                .iter()
                .map(|col| {
                    let indice = columnas_csv
                        .iter()
                        .position(|col_csv| col_csv == col)
                        .unwrap_or(0);
                    linea[indice].as_str()
                })
                .collect();
            println!("{}", linea.join(","));
        }
    }
    Ok(())
}

pub fn aplicar_select(ruta: &String, select: &Select) -> Result<(), Box<dyn Error>> {
    let (lineas, columnas_csv) = abrir_archivo(ruta)?;
    if select.columnas.contains(&"*".to_string()) && select.columnas.len() > 1 {
        return Err(Box::new(ErrorPrograma::new(
            ErrorTipo::Column,
            "No se puede seleccionar todas las columnas '*' y otras columnas al mismo tiempo.",
        )));
    } else if !select.columnas.contains(&"*".to_string()) {
        verificar_existen_columnas_query(&select.columnas, &columnas_csv)?;
    }
    if select.where_clauses.is_some() {
        verificar_existen_columnas_where_clause(&select.where_clauses, &columnas_csv)?;
    }

    let lineas_filtradas =
        filtrar_lineas_que_cumplen_con_where_clause(lineas, select, &columnas_csv)?;

    let lineas_filtradas = ordenar_lineas_select(select, lineas_filtradas, &columnas_csv)?;

    if select.columnas[0] == "*" {
        println!("{}", columnas_csv.join(","));
    } else {
        println!("{}", select.columnas.join(","));
    }

    seleccionar_y_escribir_columnas_pedidas(lineas_filtradas, select, &columnas_csv)?;

    Ok(())
}

fn aplicar_filtro_extend(
    valores: &[String],
    columnas_csv: &[String],
    where_clauses: &[WhereClause],
    operadores_logicos: &[LogicalOperators],
) -> Result<(bool, Vec<bool>), Box<dyn Error>> {
    let mut resultado_and = true;
    let mut or_conditions = Vec::new();
    let mut evaluando_and = false;
    for (index, clause) in where_clauses.iter().enumerate() {
        let resultado_actual = cumple_condicion(
            &valores.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
            columnas_csv,
            clause,
        )?;
        match operadores_logicos.get(index) {
            Some(LogicalOperators::And) => {
                if evaluando_and {
                    resultado_and = resultado_and && resultado_actual;
                } else {
                    resultado_and = resultado_actual;
                    evaluando_and = true;
                }
            }
            Some(LogicalOperators::Or) => {
                if evaluando_and {
                    or_conditions.push(resultado_and);
                    evaluando_and = false;
                }
            }
            None => {
                if evaluando_and {
                    resultado_and = resultado_and && resultado_actual;
                } else {
                    resultado_and = resultado_actual;
                }
                or_conditions.push(resultado_and);
            }
        }
    }
    Ok((resultado_and, or_conditions))
}

fn aplicar_filtro(
    valores: &[String],
    columnas_csv: &[String],
    where_clauses: &[WhereClause],
    operadores_logicos: &[LogicalOperators],
) -> Result<bool, Box<dyn Error>> {
    if where_clauses.is_empty() {
        return Ok(true);
    }

    let (resultado_and, or_conditions) =
        aplicar_filtro_extend(valores, columnas_csv, where_clauses, operadores_logicos)?;

    // Evaluar las condiciones OR
    let mut resultado_final = resultado_and;
    for cond in or_conditions {
        resultado_final = resultado_final || cond;
    }
    Ok(resultado_final)
}

fn cumple_condicion(
    valores: &[&str],
    columnas_csv: &[String],
    where_clause: &WhereClause,
) -> Result<bool, Box<dyn Error>> {
    let (valor1_string, valor1_int, es_numero1) =
        obtener_valor(valores, columnas_csv, &where_clause.valor1)?;
    let (valor2_string, valor2_int, es_numero2) =
        obtener_valor(valores, columnas_csv, &where_clause.valor2)?;
    let resultado = if es_numero1 && es_numero2 {
        match where_clause.comparacion {
            Comparaciones::Igual => valor1_int == valor2_int,
            Comparaciones::Diferente => valor1_int != valor2_int,
            Comparaciones::Mayor => valor1_int > valor2_int,
            Comparaciones::Menor => valor1_int < valor2_int,
            Comparaciones::MayorIgual => valor1_int >= valor2_int,
            Comparaciones::MenorIgual => valor1_int <= valor2_int,
        }
    } else {
        match where_clause.comparacion {
            Comparaciones::Igual => valor1_string == valor2_string,
            Comparaciones::Diferente => valor1_string != valor2_string,
            Comparaciones::Mayor => valor1_string > valor2_string,
            Comparaciones::Menor => valor1_string < valor2_string,
            Comparaciones::MayorIgual => valor1_string >= valor2_string,
            Comparaciones::MenorIgual => valor1_string <= valor2_string,
        }
    };
    if where_clause.es_not {
        Ok(!resultado)
    } else {
        Ok(resultado)
    }
}

fn obtener_valor(
    valores: &[&str],
    columnas_csv: &[String],
    valor: &str,
) -> Result<(String, usize, bool), Box<dyn Error>> {
    let valor = valor.trim_matches('\'');

    if columnas_csv.contains(&valor.to_string()) {
        let indice = columnas_csv
            .iter()
            .position(|col| col == valor)
            .ok_or(Box::new(ErrorPrograma::new(
                ErrorTipo::Column,
                "La coumna no se encuentra.",
            )))?;
        Ok((valores[indice].to_string(), 0, false))
    } else if es_int(valor) {
        Ok((valor.to_string(), valor.parse::<usize>()?, true))
    } else {
        Ok((valor.to_string(), 0, false))
    }
}

fn verificar_si_linea_cumple_condicion(
    linea: &str,
    where_clauses: &Option<Vec<WhereClause>>,
    operacion_logica: &Option<Vec<LogicalOperators>>,
    columnas_csv: &[String],
) -> Result<(bool, Vec<String>), Box<dyn Error>> {
    let valores: Vec<String> = linea.split(',').map(|s| s.trim().to_string()).collect();

    let cumple_condicion = if let Some(ref condiciones) = where_clauses {
        if !condiciones.is_empty() {
            if let Some(ref operadores_logicos) = operacion_logica {
                aplicar_filtro(&valores, columnas_csv, condiciones, operadores_logicos)?
            } else {
                aplicar_filtro(&valores, columnas_csv, condiciones, &[])?
            }
        } else {
            true
        }
    } else {
        true
    };
    Ok((cumple_condicion, valores))
}

fn actualizar_valores_fila(
    update: &Update,
    mut valores: Vec<String>,
    columnas_csv: &[String],
) -> Result<Vec<String>, Box<dyn Error>> {
    for set_clause in update.set.split(',') {
        let partes_set: Vec<&str> = set_clause.split('=').map(|s| s.trim()).collect();
        if partes_set.len() == 2 {
            let columna = partes_set[0];
            let mut valor_a_cambiar = partes_set[1].to_string();

            if let Some(indice) = columnas_csv.iter().position(|col| col == columna) {
                if valor_a_cambiar.starts_with('\'') && valor_a_cambiar.ends_with('\'') {
                    valor_a_cambiar = valor_a_cambiar[1..valor_a_cambiar.len() - 1].to_string();
                }
                valores[indice] = valor_a_cambiar.to_string();
            } else {
                let descripcion =
                    format!("La columna {} no fue existe en el archivo csv.", columna);
                return Err(Box::new(ErrorPrograma::new(
                    ErrorTipo::Column,
                    &descripcion,
                )));
            }
        } else {
            return Err(Box::new(ErrorPrograma::new(
                ErrorTipo::Syntax,
                "La sintaxis es inválida en la SET clause.",
            )));
        }
    }
    Ok(valores)
}

pub fn aplicar_update(ruta: &String, update: &Update) -> Result<(), Box<dyn Error>> {
    let (lineas, columnas_csv) = abrir_archivo(ruta)?;
    if update.where_clauses.is_some() {
        verificar_existen_columnas_where_clause(&update.where_clauses, &columnas_csv)?;
    }
    let archivo_temporal = File::create("datos/temporal.csv")?;
    let mut temporal_writer = BufWriter::new(archivo_temporal);
    writeln!(temporal_writer, "{}", columnas_csv.join(","))?;

    for linea in lineas {
        let linea = linea?;
        let (cumple_condicion, valores) = verificar_si_linea_cumple_condicion(
            &linea,
            &update.where_clauses,
            &update.operacion_logica,
            &columnas_csv,
        )?;

        if cumple_condicion {
            let valores = actualizar_valores_fila(update, valores, &columnas_csv)?;
            let nueva_linea = valores.join(",");

            writeln!(temporal_writer, "{}", nueva_linea)?;
        } else {
            writeln!(temporal_writer, "{}", linea)?;
        }
    }
    rename("datos/temporal.csv", ruta)?;
    drop(temporal_writer);
    Ok(())
}

pub fn aplicar_delete(ruta: &String, delete: &Delete) -> Result<(), Box<dyn Error>> {
    let (lineas, columnas_csv) = abrir_archivo(ruta)?;
    if delete.where_clauses.is_some() {
        verificar_existen_columnas_where_clause(&delete.where_clauses, &columnas_csv)?;
    } else {
        Err(Box::new(ErrorPrograma::new(
            ErrorTipo::Syntax,
            "No hay una WHERE clause en la consulta DELETE.",
        )))?
    }
    let archivo_temporal = File::create("datos/temporal.csv")?;
    let mut temporal_writer = BufWriter::new(archivo_temporal);

    writeln!(temporal_writer, "{}", columnas_csv.join(","))?;

    for linea in lineas {
        let linea = linea?;
        let (cumple_condicion, _) = verificar_si_linea_cumple_condicion(
            &linea,
            &delete.where_clauses,
            &delete.operacion_logica,
            &columnas_csv,
        )?;
        if !cumple_condicion {
            writeln!(temporal_writer, "{}", linea)?;
        }
    }

    rename("datos/temporal.csv", ruta)?;
    temporal_writer.flush()?;
    drop(temporal_writer);

    Ok(())
}

fn escribir_linea(
    linea: &str,
    file: &mut File,
    columnas_csv: &[String],
    columnas_insert: &[&str],
) -> Result<(), Box<dyn Error>> {
    let valores: Vec<&str> = linea.split(',').map(|s| s.trim()).collect();
    let mut linea_actual = String::new();

    for columna_c in columnas_csv.iter() {
        if let Some(pos) = columnas_insert.iter().position(|&col| col == *columna_c) {
            let mut valor_actual = valores.get(pos).unwrap_or(&"").to_string();
            if valor_actual.starts_with('\'') && valor_actual.ends_with('\'') {
                valor_actual = valor_actual[1..valor_actual.len() - 1].to_string();
            }
            linea_actual.push_str(&valor_actual);
        }
        linea_actual.push(',');
    }

    if linea_actual.ends_with(',') {
        linea_actual.pop();
    }

    writeln!(file, "{}", linea_actual)?;
    Ok(())
}

pub fn aplicar_insert(
    ruta: &String,
    nombre_columnas_insertar: &str,
    valores_a_insertar: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let (_, columnas_csv) = abrir_archivo(ruta)?;

    let columnas_insert: Vec<&str> = nombre_columnas_insertar
        .split(',')
        .map(|s| s.trim())
        .collect();

    verificar_existen_columnas_query(
        &columnas_insert
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>(),
        &columnas_csv,
    )?;

    let mut file = OpenOptions::new().append(true).open(ruta)?;

    for clause in valores_a_insertar.iter() {
        escribir_linea(clause, &mut file, &columnas_csv, &columnas_insert)?;
    }
    Ok(())
}
