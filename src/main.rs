use std::io::{
    self,
    Write,
    Read,
};

//Funcion para calcular el valor de xm o ym
fn med(m1: f32, m2: f32) -> f32 {
    (m1 + m2) / 2.0
}

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
    print!("Hola y bienvenido a rustcalcul\nSeleccione la operacion que desee realizar:\n1. Calcular la distancia entre dos puntos.\n2. Calcular la pendiente de dos puntos.\n3. Calcular el punto medio de dos puntos.\n4. Calcular la velocidad de un objeto (en m/s).\nSu opcion:\t");

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
            
            println!("El resultado de sqrt(({} - {})^2 + ({} - {})^2) es:\t{}", val[0], val[1], val[2], val[3], dist(val[0], val[1], val[2], val[3]));
            pause();
        },

        2 => {
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
            println!("El resultado de ({} - {}) / ({} - {}):\t{}", val[3], val[2], val[1], val[0], pend(val[0], val[1], val[2], val[3]));
            pause();
        },

        3 => {
            let mut val = [0.0, 0.0, 0.0, 0.0];
            input = String::from("");
            println!("Ha seleccionado calcular el punto medio de dos puntos");
            println!("Para calcular la distancia de dos puntos son necesarias las coordenadas de esos dos puntos");
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
            println!("El punto medio de: ({}, {}) y ({}, {}) es: ({}, {})", val[0], val[2], val[1], val[3], med(val[0], val[1]), med(val[2], val[3]));
            pause();
        },
        
        4 => {
            input = String::from(""); //Vaciar entrada
            println!("Haz seleccionado calcular la velocidad de un objeto (en m/s)");
            println!("Para calcular la velocidad de un objeto es necesario saber la distancia en metros que recorrio el objeto y el tiempo en segundos que le tomo cubrir esa distancia\nSu formula es: distancia/tiempo\nIngrese la distancia recorrida (en metros)");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let mut dis = input.trim().parse::<f32>().unwrap();
            println!("Ingrese el tiempo que le tomo al objeto recorrer la distancia");
            io::stdout().flush().unwrap();
            input = String::from("");
            io::stdin().read_line(&mut input).unwrap();
            let mut tiempo = input.trim().parse::<f32>().unwrap();
            println!("La velocidad de un objeto que recorrio {}m en {}s es de {}m/s", dis, tiempo, dis/tiempo);
            pause();
        },

        5 => {
            println!("To do:\nAñadir derivacion.\n");
        }

        _=>{
            println!("No ingresaste una opcion valida :(");
            std::process::exit(1);
        },
    }
}
