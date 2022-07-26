use std::io;

fn main() {
    println!("Please enter the nth term of the Fibonacci Sequence you'd like output: ");
    
    let mut term = String::new();

    io::stdin().read_line(&mut term).expect("Please enter a number");

    let term = match term.trim().parse::<u8>() {
        Ok(num) => {
            println!("Thanks for inputting a number: {num}");
            fib(num);
        },
        Err(..) => {
            println!("Thanks for not listening to instructions: {term}");
            return;
        }
    };
}

fn fib (x: u8) {
    let mut vec = vec![1, 1];
    let y = x - 1;

    if y == 0 || y == 1 {
        println!("{}", vec[y as usize]);
    }

    for number in 2..=y {
        let mut i = number as usize;
        vec.push(vec[i-2] + vec[i-1]);
    }

    println!("{}", vec[y as usize]);
}