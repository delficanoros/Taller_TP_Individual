use std::fs::{copy, create_dir_all, remove_dir_all};
use taller_tp_individual::query_identifier::analisar_query;
use taller_tp_individual::query_identifier::Comparacion;
use taller_tp_individual::query_identifier::Delete;
use taller_tp_individual::query_identifier::LogicalOperators;
use taller_tp_individual::query_identifier::WhereClause;
use taller_tp_individual::read_file::crear_escribir_archivo_temporal_delete;

// Test 18: Query tipo Delete elimina un registro
#[test]
fn test_delete_funciona_correctamente() {
    let ruta = "tests/test_delete/datos".to_string();
    copy(
        "tests/test_delete/datos/clientes_test_delete_copia.csv",
        "tests/test_delete/datos/clientes_test_delete1.csv",
    )
    .expect("No se pudo copiar el archivo");
    let query = "DELETE FROM clientes_test_delete1 WHERE id = 4;";
    let resultado = analisar_query(&ruta, query);
    println!("{:?}", resultado);
    assert!(resultado.is_ok());
}

// Test 19: Query tipo Delete elimina muchos registros
// Query: "DELETE FROM clientes_test_delete2 WHERE id > 4;"
#[test]
fn test_delete_elimina_multiples() {
    let ruta = "tests/test_delete/datos/clientes_test_delete2.csv";

    let where_clause =
        WhereClause::new("id".to_string(), Comparacion::Mayor, "4".to_string(), false);
    let where_clauses = Some(vec![where_clause]);
    let order_by = None;
    let delete = Delete::new("clientes_test_delete2", where_clauses, order_by);

    let resultado = crear_escribir_archivo_temporal_delete(
        &"datos/temporal2.csv".to_string(),
        &delete,
        &ruta.to_string(),
    );
    assert!(resultado.is_ok());
}

// Test 20: Se devuelve error en caso de querer eliminar de una where clause incorrecta
#[test]
fn test_delete_columna_where_no_existe() {
    let ruta = "tests/test_delete/datos".to_string();
    copy(
        "tests/test_delete/datos/clientes_test_delete_copia3.csv",
        "tests/test_delete/datos/clientes_test_delete3.csv",
    )
    .expect("No se pudo copiar el archivo");
    let query = "DELETE FROM clientes_test_delete3 WHERE gmail != 'prueba@gmail.com';";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 21: Se puede eliminar verificando varías where clauses
// Query: "DELETE FROM clientes_test_delete4 WHERE id = 2 AND nombre = 'Ana';";
#[test]
fn test_delete_varias_where_clause() {
    let ruta = "tests/test_delete/datos/clientes_test_delete4.csv";
    let where_clause =
        WhereClause::new("id".to_string(), Comparacion::Igual, "2".to_string(), false);
    let where_clause2 = WhereClause::new(
        "nombre".to_string(),
        Comparacion::Igual,
        "'Ana'".to_string(),
        false,
    );
    let where_clauses = Some(vec![where_clause, where_clause2]);
    let operacion_logica = Some(vec![LogicalOperators::And]);
    let delete = Delete::new("clientes_test_delete2", where_clauses, operacion_logica);
    let resultado = crear_escribir_archivo_temporal_delete(
        &"datos/temporal3.csv".to_string(),
        &delete,
        &ruta.to_string(),
    );
    println!("{:?}", resultado);
    assert!(resultado.is_ok());
}

// Test 22: No se puede eliminar de un archivo vacío
#[test]
fn test_delete_archivo_vacío() {
    let ruta = "tests/test_delete/datos".to_string();
    let query = "DELETE FROM vacio WHERE id = 2;";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}
