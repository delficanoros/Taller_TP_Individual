use taller_tp_individual::query_identifier::analisar_query;
use taller_tp_individual::query_identifier::Select;

/*
 * Todos los archivos son testeados sobre los csv que se encuentran en la carpeta test/datos_test,
 * así cuando se van modificando no afectan a las otras base de datos.
 * Para los test de queries tipo select utilizo el archivo clientes.csv ya que no se modifican los datos.
 * Para los otros test, cada uno tiene su respectivo archivo.
 */
// Test 01: Se verifica que una query tipo select no devuelve error y se ejecuta correctamente.
#[test]
fn test_select_funciona_correcta() {
    let ruta = "tests/test_select/datos".to_string();
    let query = "SELECT * FROM clientes";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 02: Se verifica que una query tipo select con Where y Order By no devuelve error y se ejecuta correctamente.
#[test]
fn test_select_where_orderby() {
    let ruta = "tests/test_select/datos".to_string();
    let query = "SELECT id, nombre, apellido FROM clientes WHERE id >= 3 AND nombre = 'Carlos' ORDER BY apellido";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 03: Si se ingresa de manera incorrecta la consulta, devuelve error
#[test]
fn test_select_query_mal_escritos() {
    let ruta = "tests/test_select/datos".to_string();
    let query = "SELEECT id, nombre, apellido FROM clientes WHERE id >= 3 AND nombre = 'Carlos' ORDER BY apellido";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 04: Si el nombre de la tabla es incorrecto, devuelve error
#[test]
fn test_select_nombre_tabla_incorrecto() {
    let ruta = "tests/test_select/datos".to_string();
    let query = "SELECT *, id, nombre, apellido FROM clientes";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 05: Devuelve error si la tabla se encuentra vacía
#[test]
fn test_select_tabla_vacia() {
    let ruta = "tests/test_select/datos".to_string();
    let query = "SELECT id, nombre, apellido FROM vacio";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 06: No se puede mostrar las columnas de una tabla que no existe
#[test]
fn test_select_tabla_no_existe() {
    let ruta = "tests/test_select/datos".to_string();
    let query = "SELECT * FROM clientess";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 07: No se puede select sobre una columna que no existe en la tabla
#[test]
fn test_select_columna_no_existe() {
    let ruta = "tests/test_select/datos".to_string();
    let query = "SELECT nombre, segundo_nombre FROM clientes WHERE nombre = 'Pedro'";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 0: No se puede aplicar where una columna que no existe en la tabla
#[test]
fn test_select_where_columna_no_existe() {
    let ruta = "tests/test_select/datos".to_string();
    let query = "SELECT * FROM clientes WHERE segundo_nombre = 'Pedro'";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 08: Muchas where clauses variadas en la query
#[test]
fn test_select_muchas_where_clauses() {
    let ruta = "tests/test_select/datos".to_string();
    let query = "SELECT * FROM appointment WHERE 3 <= Physician OR ExaminationRoom = 'A' AND Physician = 1 OR Patient >= 100000004 ORDER BY AppointmentID;";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 08: Muchas where clauses de tipo and y una or en la query
#[test]
fn test_select_muchas_where_clauses_con_and() {
    let ruta = "tests/test_select/datos".to_string();
    let query = "SELECT * FROM appointment WHERE Physician >= 3 AND ExaminationRoom = 'A' AND Physician = 1 OR Patient >= 100000004 AND Physician != 'C'";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 08: No se puede aplicar order by sobre una columna que no existe en la tabla
#[test]
fn test_select_order_by_columna_no_existe() {
    let ruta = "tests/test_select/datos".to_string();
    let query = "SELECT * FROM clientes ORDER BY segundo_nombre";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 09: Se puede ordenar por todas las columnas
#[test]
fn test_select_se_puede_ordenar_por_todas_las_columnas() {
    let ruta = "tests/test_select/datos".to_string();
    let query = "SELECT * FROM clientes ORDER BY id ASC, nombre DESC, apellido, email DESC";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

//Test 28: Se verifica que se parsee correctamente la query de select
#[test]
fn test_select_parsear_query_funciona_correctamente() {
    let query = "SELECT id, nombre, apellido FROM clientes WHERE id >= 3 AND nombre = 'Carlos' ORDER BY apellido"
            .to_string();
    let posicion_from = query.find("FROM").ok_or("Test failed");
    let columnas = [
        "id".to_string(),
        "nombre".to_string(),
        "apellido".to_string(),
    ]
    .to_vec();
    assert!(Select::crear_select(&query, posicion_from.unwrap(), columnas).is_ok());
}
