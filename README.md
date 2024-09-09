**Taller de Programación: TP Individual**

Nombre: María Delfina Cano Ros Langrehr

Padrón: 109338 


**SQL Rústico**

Comandos disponibles: `Select, Update, Insert y Delete`

Para realizar consultas, ejecutar: `cargo run -- ruta/a/tablas "query" > output.csv
En el caso de la consulta SELECT se puede utilizar '> archivo_salida.csv' en caso de que se quiera imprimir la salida en un csv aparte, en caso de que no se indique se imprimirá en la terminal


Ejemplos Select:
```py
1. SELECT * FROM clientes;

2. SELECT id, nombre, apellido FROM clientes WHERE id >= 3 AND nombre = 'Carlos' ORDER BY apellido;

3. SELECT nombre, apellido FROM clientes ORDER BY nombre, apellido DESC, id ASC;

4. SELECT id, nombre WHERE 2 <= id AND apellido = nombre;

5. SELECT * FROM appointment WHERE Physician >= 3 AND ExaminationRoom = 'A' AND Physician = 1 OR Patient >= 100000004 AND Physician != 'C';
```

Ejemplos Update:
```py
6. UPDATE clientes SET nombre = 'Pedro' WHERE id = 1

7. UPDATE clientes SET nombre = 'Pedro', apellido = 'Cano', email = 'pedro.cano@gmail.com' WHERE id = 2

8. UPDATE clientes SET nombre = 'Alfonso' #cambia todos los valores si no hay where clause
```

Ejemplos Insert:
```py
9. INSERT INTO clientes (id, nombre, apellido, email) VALUES (7, 'Pedro', 'Cano', 'pedro.cano@gmail.com');

10. INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (111, 6, 'Laptop', 3), (112, 4, 'Cargador', 4);

```

Ejemplos Delete:
```py
11. DELETE FROM ordenes WHERE id > 4

12. DELETE FROM ordenes WHERE producto != 'Laptop' AND producto != 'Cargador'
```



