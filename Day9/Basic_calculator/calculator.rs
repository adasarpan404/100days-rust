use std::io;

fn main() {
    println!("Welcome to the calculator!");

    loop {
        let mut input = String::new();
        println!("Enter an expression: ");
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "quit" {
            break;
        }

        let mut parts = input.split_whitespace();
        let x = parts.next().unwrap().parse::<f64>().unwrap();
        let op = parts.next().unwrap();
        let y = parts.next().unwrap().parse::<f64>().unwrap();

        let result = match op {
            "+" => x + y,
            "-" => x - y,
            "*" => x * y,
            "/" => x / y,
            _ => {
                println!("Invalid operator!");
                continue;
            }
        };

        println!("Result: {}", result);
    }
}
