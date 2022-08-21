use colored::*;
use std::io::{
    self,
    Write,
    Read,
};

//Pausar
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Presiona enter...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
//Calcular la pendiente de dos puntos
fn pend(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
    (y2 - y1) / (x2 - x1)
}
//Calcular la distancia entre dos puntos
fn dist(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
    let mut ax = [x2 - x1, y2 - y1, 0.0];
    ax[0] = f32::powi(ax[0], 2);
    ax[1] = f32::powi(ax[1], 2);
    ax[2] = ax[0] + ax[1];
    ax[2].sqrt()
}

fn main() {
    print!("Hola y bienvenido a rustcalcul\nSeleccione la operacion que desee realizar:\n[1] Calcular la distancia entre dos puntos.\n[2] Calcular la pendiente de dos puntos.\n[3] Calcular el punto medio de dos puntos.\n[4] Calcular la velocidad de un objeto.\nSu opcion:\t");

    //Leer entrada y guardarla en la variable input en el tipo de dato String
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    //Convertir entrada a entero de 32 bits
    let mut select = input.trim().parse::<i32>().unwrap();

    //Funcion match para llamar la operacion seleccionada
    match select {
        1 => {
            //Empieza generacion de entrada para la operacion necesaria
            input = String::from(""); //Vaciar entrada

            let mut val = [0.0, 0.0, 0.0, 0.0]; //Valores necesarios para la generacion de entrada

            println!("Ha seleccionado calcular la distancia de dos puntos \n");
            println!("Para calcular la distancia de dos puntos son necesarias las coordenadas de ambos puntos, su formula es:\tsqrt((x2 - x1)^2 + (y2 - y1)^2)");
            //Bucle para pedir la entrada necesaria
            let mut i = 0;
            while val.len() > i {

                if i <= 1 {
                    println!("ingrese el valor de x{}:\t", i+1);
                    io::stdout().flush().unwrap();
                }
                else{
                    println!("ingrese el valor de y{}:\t", i-1);
                    io::stdout().flush().unwrap();
                    }

                io::stdin().read_line(&mut input).unwrap();
                val[i] = input.trim().parse::<f32>().unwrap();

                input = String::from(""); //Vaciar entrada
                i = i + 1;
            }
            
            println!("El resultado de sqrt(({} - {})^2 + ({} - {})^2) es:\t{}", val[0], val[1], val[2], val[3], dist(val[0], val[1], val[2], val[3]).to_string().green());
            pause();
        },

        2=>{
            input = String::from(""); //Vaciar entrada
            let mut val = [0.0, 0.0, 0.0, 0.0];

            println!("Ha seleccionado calcular la pendiente de dos puntos");
            println!("Para calcular la  pendiente de dos puntos son necesarias las coordendas de esos dos puntos, su formula es:\t(y2 - y1) / (x2 - x1)");
            let mut i = 0;
            while val.len() > i {
                if i <= 1 {
                    println!("ingrese el valor de x{}:\t", i+1);
                    io::stdout().flush().unwrap();
                }
                else{
                    println!("ingrese el valor de y{}:\t", i-1);
                    io::stdout().flush().unwrap();
                    }

                io::stdin().read_line(&mut input).unwrap();
                val[i] = input.trim().parse::<f32>().unwrap();

                input = String::from(""); //Vaciar entrada
                i = i + 1;
            }
            println!("El resultado de ({} - {}) / ({} - {}):\t{}", val[3], val[2], val[1], val[0], pend(val[0], val[1], val[2], val[3]).to_string().green());
            pause();
        },
        
        3|4=>{
            println!("En proceso :D");
            std::process::exit(1);
        },

        _=>{
            println!("No ingresaste una opcion valida :(");
            std::process::exit(1);
        },
    }
}