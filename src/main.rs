use colored::*;
use std::io::Write;

fn main() {
    print!("Hola y bienvenido a rustcalcul\nSeleccione la operacion que desee realizar:\n");

    //Leer entrada y guardarla en la variable input en el tipo de dato String
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();

    //Convertir entrada a entero de 32 bits
    let mut select = input.trim().parse::<i32>().unwrap_or_else(|_| {
        drop(input);
        println!("La entrada no era del tipo entero");
        std::process::exit(1);
    });

    //Funcion match para llamar la operacion seleccionada
    match select {
        1=>println!("hola"),
        2=>{
            println!("hola2");
            println!("hola");
        },
        _=>{
            println!("No ingresaste una opcion valida");
            std::process::exit(1);
        },
    }
}
