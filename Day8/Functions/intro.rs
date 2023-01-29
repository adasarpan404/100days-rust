// I am writing fizzbuzz

fn main() {
    fizzbuzzTo(100);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if isDivisibleBy(n, 15) {
        println!("fizzbuzz");
    } else if isDivisibleBy(n, 3) {
        println!("fizz");
    } else if isDivisibleBy(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n)
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
