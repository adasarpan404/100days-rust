use std::io;

fn main() {
    println!("Enter number of elements: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();
    let mut numbers = vec![0; n];

    println!("Enter {} numbers separated by a space: ", n);

    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let input_numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for (i, number) in input_numbers.iter().enumerate() {
        numbers[i] = *number;
    }

    println!("Numbers in the array: {:?}", numbers);
}
