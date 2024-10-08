use taller_tp_individual::query_identifier::analisar_query;
use taller_tp_individual::query_identifier::Insert;

// Test 22: Se testea insertar un solo registro
#[test]
fn test_insert_un_registro() {
    let ruta = "tests/test_insert/datos".to_string();
    let query = "INSERT INTO clientes_test_insert (id, nombre, apellido, email) VALUES (7, 'Pedro', 'Cano', 'pedro.cano@gmail.com');";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

// Test 22: Se testea insertar varios registrs
#[test]
fn test_insert_varios_registros() {
    let ruta = "tests/test_insert/datos".to_string();
    let query = "INSERT INTO clientes_test_insert (id, nombre, apellido, email) VALUES (8, 'Manuel', 'Alonso', 'manu.alonso@gmail.com'),
    (9,'Lucas', 'Bono', 'lucas.bono@gmail.com'),
    (10, 'Abril', 'Comesaña', 'abril.comesaña@gmail.com');";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_ok());
}

#[test]
// Test 23: No se puede insertar en una tabla que no existe
fn test_insert_tabla_no_existente() {
    let ruta = "tests/test_insert/datos".to_string();
    let query = "INSERT INTO clientesss (id, nombre, apellido, email) VALUES (7, 'Pedro', 'Cano', 'pedro.cano@gmail.com');";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

#[test]
// Test 24: No se puede insertar si una columna no existe
fn test_insert_columna_no_existente() {
    let ruta = "tests/test_insert/datos".to_string();
    let query = "INSERT INTO clientes_test_insert (id, segundo, apellido, email) VALUES (7, 'Pedro', 'Cano', 'pedro.cano@gmail.com');";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
}

#[test]
// Test 25: Sintaxis incorrecta en la query
fn test_insert_sintaxis_incorrecta() {
    let ruta = "tests/test_insert/datos".to_string();
    let query =
        "INSERT INTO clientes_test_insert id VALUES (7, 'Pedro', 'Cano', 'pedro.cano@gmail.com');";
    let resultado = analisar_query(&ruta, query);
    assert!(resultado.is_err());
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
