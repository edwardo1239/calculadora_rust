use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();

        println!("Ingrese la operacion que desea realizar o q para salir");

        io::stdout().flush().expect("Error al limpiar el buffer");

        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la linea");

        let input = input.trim();

        if input == "q" {
            break;
        }

        let datos: Vec<&str> = input.split_whitespace().collect();

        if datos.len() < 3 {
            println!("Por favor, ingrese al menos dos números y un operador.");
            continue;
        }

        let num1: f64 = match datos[0].parse() {
            Ok(num) => num,
            Err(_) =>{
                println!("Error: {} no es un numero valido", datos[0]);
                continue;
            }
        };

        
        let num2: f64 = match datos[2].parse() {
            Ok(num) => num,
            Err(_) =>{
                println!("Error: {} no es un numero valido", datos[2]);
                continue;
            }
        };

        match datos[1] {
            "+" =>  println!("Operacion suma: {} + {} = {}", num1, num2, num1 + num2),
            "-" =>  println!("Operacion resta: {} - {} = {}", num1, num2, num1 - num2),
            _ =>  println!("Funcion no soportada")
            
        }

    }
    println!("Programa terminado");
}
