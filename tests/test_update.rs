use std::fs::{copy, create_dir_all, remove_dir_all};
use std::vec;
use taller_tp_individual::query_identifier::analisar_query;
use taller_tp_individual::query_identifier::Comparacion;
use taller_tp_individual::query_identifier::LogicalOperators;
use taller_tp_individual::query_identifier::Update;
use taller_tp_individual::query_identifier::WhereClause;
use taller_tp_individual::read_file::crear_escribir_archivo_temporal_update;

// Test 10: Se verifica que una query tipo update no devuelve error y se ejecuta correctamente.
// query = "UPDATE clientes_test_update1 SET nombre = 'Pedro' WHERE id = 1";
#[test]
fn test_update_funciona_correctamente() {
    let ruta = "tests/test_update/datos/clientes_test_update1.csv".to_string();
    let where_clauses = Some(vec![WhereClause::new(
        "id".to_string(),
        Comparacion::Igual,
        "1".to_string(),
        false,
    )]);
    let operaciones = None;
    let set = "nombre = 'Pedro'";
    let update = Update::new("clientes_test_update3", set, where_clauses, operaciones);
    let resultado =
        crear_escribir_archivo_temporal_update(&ruta, &update, &"datos/temporal5.csv".to_string());
    println!("{:?}", resultado);
    assert!(resultado.is_ok());
}
// Test 11: No hay error si se quieren modificar varios valores de la tabla
#[test]
fn test_update_funciona_correctamente_con_varias_condiciones() {
    let ruta_base = "tests/test_update/datos_temp";
    let ruta = format!("{}/test11", ruta_base);

    // Crear un directorio temporal para este test
    create_dir_all(&ruta).expect("No se pudo crear el directorio temporal");

    // Copiar el archivo a un nombre temporal específico para este test
    copy(
        "tests/test_update/datos/clientes_test_update_copia2.csv",
        format!("{}/clientes_test_update2.csv", &ruta),
    )
    .expect("No se pudo copiar el archivo");

    let query = "UPDATE clientes_test_update2 SET nombre = 'Pedro', apellido = 'Cano', email = 'pedro.cano@gmail.com' WHERE id = 2";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());

    // Limpiar los archivos temporales después del test
    remove_dir_all(&ruta).expect("No se pudo eliminar el directorio temporal");
}

// Test 12: No se puede insertar en una tabla vacía
#[test]
fn test_update_archivo_vacio() {
    let ruta = "tests/test_update/datos".to_string();
    let query = "UPDATE vacio SET nombre = 'Pedro', apellido = 'Cano', id = 5 WHERE id = 2";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 13: Se puede actualizar verificando varías clauses where
// Query; "UPDATE clientes_test_update3 SET nombre = 'Pedro', apellido = 'Cano', id = 5 WHERE id = 3 AND nombre = 'Carlos';"
#[test]
fn test_update_muchas_where_clauses() {
    let ruta = "tests/test_update/datos/clientes_test_update3.csv".to_string();
    let where_clauses = Some(vec![
        WhereClause::new("id".to_string(), Comparacion::Igual, "3".to_string(), false),
        WhereClause::new(
            "nombre".to_string(),
            Comparacion::Igual,
            "'Carlos'".to_string(),
            false,
        ),
    ]);
    let operaciones = Some(vec![LogicalOperators::And]);
    let set = "nombre = 'Pedro', apellido = 'Cano', id = 5";
    let update = Update::new("clientes_test_update3", set, where_clauses, operaciones);
    let resultado =
        crear_escribir_archivo_temporal_update(&ruta, &update, &"datos/temporal4.csv".to_string());
    println!("{:?}", resultado);
    assert!(resultado.is_ok());
}
// Test 14: No se puede actualizar una tabla que no existe
#[test]
fn test_update_tabla_no_existente() {
    let ruta = "tests/test_update/datos".to_string();
    let query = "UPDATE clientess SET nombre = 'Pedro', apellido = 'Cano', id = 5 WHERE id = 3 AND nombre = 'Carlos'";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 15: No se puede actualizar una columna que no existe
#[test]
fn test_update_columna_no_existe() {
    let ruta = "tests/test_update/datos".to_string();
    copy(
        "tests/test_update/datos/clientes_test_update_copia4.csv",
        "tests/test_update/datos/clientes_test_update4.csv",
    )
    .expect("No se pudo copiar el archivo");
    let query =
        "UPDATE clientes_test_update4 SET segundo_nombre = 'Pedro', apellido = 'Cano' WHERE id = 4";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 16: No se puede actualizar si ninguno de los valores es una columna
#[test]
fn test_update_set_sin_columna() {
    let ruta = "tests/test_update/datos".to_string();
    copy(
        "tests/test_update/datos/clientes_test_update_copia5.csv",
        "tests/test_update/datos/clientes_test_update5.csv",
    )
    .expect("No se pudo copiar el archivo");
    let query = "UPDATE clientes_test_update5 SET 'Ana' = 'Pedro' WHERE id = 4;";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 17: No se puede actualizar si la where clause no contiene columnas
#[test]
fn test_update() {
    let ruta = "tests/test_update/datos".to_string();
    copy(
        "tests/test_update/datos/clientes_test_update_copia6.csv",
        "tests/test_update/datos/clientes_test_update6.csv",
    )
    .expect("No se pudo copiar el archivo");
    let query = "UPDATE clientes_test_update6 SET 'Ana' = 'Pedro' WHERE 4 = 4;";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

//Test 26: Si una query update no tiene where, se modifican todos los registros
// Query: "UPDATE clientes_test_update7 SET nombre = 'Pedro'";
#[test]
fn test_update_no_hay_where_clause() {
    let ruta = "tests/test_update/datos/clientes_test_update7.csv".to_string();
    let set = "nombre = 'Pedro'";
    let update = Update::new("clientes_test_update7", set, None, None);
    let resultado =
        crear_escribir_archivo_temporal_update(&ruta, &update, &"datos/temporal6.csv".to_string());
    assert!(resultado.is_ok());
}
