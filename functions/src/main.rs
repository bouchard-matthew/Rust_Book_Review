fn main() {
    println!("Hello, world!");

    another_function(255);
}

fn another_function(x: u8) {
    println!("Another Function. Accepted Param: {x}");

    let s = {
        let y = 3;
        y + 1
    };

    println!("Value: {:?}", s);
}