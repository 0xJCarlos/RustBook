use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el numero!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Por favor introduce tu numero.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo al leer la linea");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Tu numero es: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muy pequeño!"),
            Ordering::Greater => println!("Muy grande!"),
            Ordering::Equal => {
                println!("Acertaste!");
                break;
            },
        }
    }
    
}
