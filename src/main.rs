use colored::*;
use std::io::Write;

//Calcular la distancia entre dos puntos
fn dist(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let mut ax = [x2 - x1, y2 - y1, 0.0];
    ax[0] = ax[0] * ax[0];
    ax[1] = ax[1] * ax[1];
    ax[2] = ax[0] + ax[1];
    ax[2].sqrt()
}

fn main() {
    print!("Hola y bienvenido a rustcalcul\nSeleccione la operacion que desee realizar:\n[1] Calcular la distancia entre dos puntos.\n[2] Calcular la pendiente de dos puntos.\n[3] Calcular el punto medio de dos puntos.\n[4] Calcular la velocidad de un objeto.\nSu opcion:\t");

    //Leer entrada y guardarla en la variable input en el tipo de dato String
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();

    //Convertir entrada a entero de 32 bits
    let mut select = input.trim().parse::<i32>().unwrap();

    //Funcion match para llamar la operacion seleccionada
    match select {
        1=>{
            //Empieza generacion de entrada para la operacion necesaria
            input = String::from(""); //Vaciar entrada

            let mut val = [0.0, 0.0, 0.0, 0.0]; //Valores necesarios para la generacion de entrada

            println!("Ha seleccionado calcular la distancia de dos puntos \n");
            //Bucle para pedir la entrada necesaria
            let mut i = 0;
            while val.len() > i {

                if i <= 1 {
                    println!("ingrese el valor de x{}:\t", i+1);
                    std::io::stdout().flush().unwrap();
                }
                else{
                    println!("ingrese el valor de y{}:\t", i-1);
                    std::io::stdout().flush().unwrap();
                    }

                std::io::stdin().read_line(&mut input).unwrap();
                val[i] = input.trim().parse::<f64>().unwrap();

                input = String::from(""); //Vaciar entrada
                i = i + 1;
            }
            
            println!("El resultado es:\t{}", dist(val[0], val[1], val[2], val[3]).to_string().green());
            println!("{}", input.red());
        },
        
        2|3|4=>{
            println!("En proceso :D");
            std::process::exit(1);
        },

        _=>{
            println!("No ingresaste una opcion valida :(");
            std::process::exit(1);
        },
    }
}
