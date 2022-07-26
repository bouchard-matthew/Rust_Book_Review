use std::io;

fn main() {
    println!("Please enter the nth term of the Fibonacci Sequence you'd like output (Max: 186): ");
    
    let mut term = String::new();

    io::stdin().read_line(&mut term).expect("Please enter a number");

    let _term = match term.trim().parse::<u8>() {
        Ok(num) => {
            if num <= 186 {
                println!("\nThanks for inputting a number: {num}");
                fib(num);
            } else {
                println!("\nThanks for not listening to instructions: {num}");
                return;
            }
        },
        Err(..) => {
            println!("\nThanks for not listening to instructions: {term}");
            return;
        }
    };
}

fn fib (x: u8) {
    let mut vec: Vec<u128> = vec![1, 1];
    let y = x - 1;

    if y == 0 || y == 1 {
        println!("{} \n", vec[y as usize]);
    }

    for number in 2..=y {
        let i = number as usize;
        vec.push(vec[i-2] + vec[i-1]);
    }

    println!("{} \n", vec[y as usize]);
}