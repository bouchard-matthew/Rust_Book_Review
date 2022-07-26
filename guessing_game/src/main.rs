use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello and Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(0..=5);

    loop {
        println!("Please enter a guess (number format please): ");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Unable to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too low"),
            Ordering::Equal => {
                println!("Your guess is correct. You win!\n");
                break;
            },
            Ordering::Greater => println!("Your guess is too high"),
        }
        println!("\n")
    }
}
