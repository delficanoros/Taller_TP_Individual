use taller_tp_individual::query_identifier::analisar_query;
use taller_tp_individual::query_identifier::Insert;
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
    let ruta = "tests/datos_test".to_string();
    let query = "SELECT * FROM clientes";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 02: Se verifica que una query tipo select con Where y Order By no devuelve error y se ejecuta correctamente.
#[test]
fn test_select_where_orderby() {
    let ruta = "tests/datos_test".to_string();
    let query = "SELECT id, nombre, apellido FROM clientes WHERE id >= 3 AND nombre = 'Carlos' ORDER BY apellido";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 03: Si se ingresa de manera incorrecta la consulta, devuelve error
#[test]
fn test_select_query_mal_escritos() {
    let ruta = "tests/datos_test".to_string();
    let query = "SELEECT id, nombre, apellido FROM clientes WHERE id >= 3 AND nombre = 'Carlos' ORDER BY apellido";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 04: Si el nombre de la tabla es incorrecto, devuelve error
#[test]
fn test_select_nombre_tabla_incorrecto() {
    let ruta = "tests/datos_test".to_string();
    let query = "SELECT *, id, nombre, apellido FROM clientes";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 05: Devuelve error si la tabla se encuentra vacía
#[test]
fn test_select_tabla_vacia() {
    let ruta = "tests/datos_test".to_string();
    let query = "SELECT id, nombre, apellido FROM vacio";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 06: No se puede mostrar las columnas de una tabla que no existe
#[test]
fn test_select_tabla_no_existe() {
    let ruta = "tests/datos_test".to_string();
    let query = "SELECT * FROM clientess";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 07: No se puede aplicar where una columna que no existe en la tabla
#[test]
fn test_select_columna_no_existe() {
    let ruta = "tests/datos_test".to_string();
    let query = "SELECT nombre, segundo_nombre FROM clientes WHERE nombre = 'Pedro'";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 07: No se puede aplicar where una columna que no existe en la tabla
#[test]
fn test_select_where_columna_no_existe() {
    let ruta = "tests/datos_test".to_string();
    let query = "SELECT * FROM clientes WHERE segundo_nombre = 'Pedro'";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 08: Muchas where clauses variadas en la query
#[test]
fn test_select_muchas_where_clauses() {
    let ruta = "tests/datos_test".to_string();
    let query = "SELECT * FROM appointment WHERE 3 <= Physician OR ExaminationRoom = 'A' AND Physician = 1 OR Patient >= 100000004 ORDER BY AppointmentID;";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 08: Muchas where clauses de tipo and y una or en la query
#[test]
fn test_select_muchas_where_clauses_con_and() {
    let ruta = "tests/datos_test".to_string();
    let query = "SELECT * FROM appointment WHERE Physician >= 3 AND ExaminationRoom = 'A' AND Physician = 1 OR Patient >= 100000004 AND Physician != 'C'";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 08: No se puede aplicar order by sobre una columna que no existe en la tabla
#[test]
fn test_select_order_by_columna_no_existe() {
    let ruta = "tests/datos_test".to_string();
    let query = "SELECT * FROM clientes ORDER BY segundo_nombre";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 09: Se puede ordenar por todas las columnas
#[test]
fn test_select_se_puede_ordenar_por_todas_las_columnas() {
    let ruta = "tests/datos_test".to_string();
    let query = "SELECT * FROM clientes ORDER BY id ASC, nombre DESC, apellido, email DESC";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 10: Se verifica que una query tipo update no devuelve error y se ejecuta correctamente.
#[test]
fn test_update_funciona_correctamente() {
    let ruta = "tests/datos_test".to_string();
    let query = "UPDATE clientes_test_update SET nombre = 'Pedro' WHERE id = 1";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 11: No hay error si se quieren modificar varios valores de la tabla
#[test]
fn test_update_funciona_correctamente_con_varias_condiciones() {
    let ruta = "tests/datos_test".to_string();
    let query =
        "UPDATE clientes_test_update SET nombre = 'Pedro', apellido = 'Cano', email = 'pedro.cano@gmail.com' WHERE id = 2";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 12: No se puede insertar en una tabla vacía
#[test]
fn test_update_archivo_vacio() {
    let ruta = "tests/datos_test".to_string();
    let query = "UPDATE vacio SET nombre = 'Pedro', apellido = 'Cano', id = 5 WHERE id = 2";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 13: Se puede actualizar verificando varías clauses where
#[test]
fn test_update_muchas_where_clauses() {
    let ruta = "tests/datos_test".to_string();
    let query = "UPDATE clientes_test_update SET nombre = 'Pedro', apellido = 'Cano', id = 5 WHERE id = 3 AND nombre = 'Carlos'";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}
// Test 14: No se puede actualizar una tabla que no existe
#[test]
fn test_update_tabla_no_existente() {
    let ruta = "tests/datos_test".to_string();
    let query = "UPDATE clientess SET nombre = 'Pedro', apellido = 'Cano', id = 5 WHERE id = 3 AND nombre = 'Carlos'";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 15: No se puede actualizar una columna que no existe
#[test]
fn test_update_columna_no_existe() {
    let ruta = "tests/datos_test".to_string();
    let query =
        "UPDATE clientes_test_update SET segundo_nombre = 'Pedro', apellido = 'Cano' WHERE id = 4";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 16: No se puede actualizar si ninguno de los valores es una columna
#[test]
fn test_update_set_sin_columna() {
    let ruta = "tests/datos_test".to_string();
    let query = "UPDATE clientes_test_update SET 'Ana' = 'Pedro' WHERE id = 4;";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 17: No se puede actualizar si la where clause no contiene columnas
#[test]
fn test_update() {
    let ruta = "tests/datos_test".to_string();
    let query = "UPDATE clientes_test_update SET 'Ana' = 'Pedro' WHERE 4 = 4;";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 18: Query tipo Delete elimina un registro
#[test]
fn test_delete_funciona_correctamente() {
    let ruta = "tests/datos_test".to_string();
    let query = "DELETE FROM clientes_test_delete WHERE id = 4";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 19: Query tipo Delete elimina muchos registros
#[test]
fn test_delete_elimina_multiples() {
    let ruta = "tests/datos_test".to_string();
    let query = "DELETE FROM clientes_test_delete WHERE id > 4";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 20: Se devuelve error en caso de querer eliminar de una where clause incorrecta
#[test]
fn test_delete_columna_where_no_existe() {
    let ruta = "tests/datos_test".to_string();
    let query = "DELETE FROM clientes_test_delete WHERE gmail != 'prueba@gmail.com'";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 21: Se puede eliminar verificando varías where clauses
#[test]
fn test_delete_varias_where_clause() {
    let ruta = "tests/datos_test".to_string();
    let query = "DELETE FROM clientes_test_delete WHERE id = 2 AND nombre = 'Ana'";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 22: No se puede eliminar de un archivo vacío
#[test]
fn test_delete_archivo_vacío() {
    let ruta = "tests/datos_test".to_string();
    let query = "DELETE FROM vacio WHERE id = 2";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

// Test 22: Se testea insertar un solo registro
#[test]
fn test_insert_un_registro() {
    let ruta = "tests/datos_test".to_string();
    let query = "INSERT INTO clientes_test_insert (id, nombre, apellido, email) VALUES (7, 'Pedro', 'Cano', 'pedro.cano@gmail.com');";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 22: Se testea insertar varios registrs
#[test]
fn test_insert_varios_registros() {
    let ruta = "tests/datos_test".to_string();
    let query = "INSERT INTO clientes_test_insert (id, nombre, apellido, email) VALUES (8, 'Manuel', 'Alonso', 'manu.alonso@gmail.com'), 
    (9,'Lucas', 'Bono', 'lucas.bono@gmail.com'), 
    (10, 'Abril', 'Comesaña', 'abril.comesaña@gmail.com');";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

#[test]
// Test 23: No se puede insertar en una tabla que no existe
fn test_insert_tabla_no_existente() {
    let ruta = "tests/datos_test".to_string();
    let query = "INSERT INTO clientesss (id, nombre, apellido, email) VALUES (7, 'Pedro', 'Cano', 'pedro.cano@gmail.com');";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

#[test]
// Test 24: No se puede insertar si una columna no existe
fn test_insert_columna_no_existente() {
    let ruta = "tests/datos_test".to_string();
    let query = "INSERT INTO clientes_test_insert (id, segundo, apellido, email) VALUES (7, 'Pedro', 'Cano', 'pedro.cano@gmail.com');";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

#[test]
// Test 25: Sintaxis incorrecta en la query
fn test_insert_sintaxis_incorrecta() {
    let ruta = "tests/datos_test".to_string();
    let query =
        "INSERT INTO clientes_test_insert id VALUES (7, 'Pedro', 'Cano', 'pedro.cano@gmail.com');";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

//Test 26: Si una query update no tiene where, se modifican todos los registros
#[test]
fn test_update_no_hay_where_clause() {
    let ruta = "tests/datos_test".to_string();
    let query = "UPDATE clientes_test_update SET nombre = 'Pedro'";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

//Test 27: Se verifica que se parsee correctamente la query de insert
#[test]
fn test_insert_parsear_query_funciona_correctamente() {
    let query = "INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (111, 6, 'Laptop', 3), (112, 4, 'Cargador', 4);"
        .to_string();
    let resultado = Insert::insert_parsear_query(&query);
    assert!(resultado.is_ok());
    let valor1 = "111, 6, 'Laptop', 3".to_string();
    let valor2 = "112, 4, 'Cargador', 4".to_string();
    let mut valores = Vec::new();
    valores.push(valor1);
    valores.push(valor2);
    let resultado_esperado = Insert::new("ordenes", "id, id_cliente, producto, cantidad", valores);

    assert_eq!(resultado.unwrap(), resultado_esperado);
}

//Test 27: Se verifica que se parsee correctamente la query de insert
#[test]
fn test_select_parsear_query_funciona_correctamente() {
    let query = "SELECT id, nombre, apellido FROM clientes WHERE id >= 3 AND nombre = 'Carlos' ORDER BY apellido"
        .to_string();
    let posicion_from = query.find("FROM").ok_or("Test failed");
    assert!(Select::obtener_campos(&query, posicion_from.unwrap()).is_ok());
}
