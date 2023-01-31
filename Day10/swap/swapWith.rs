fn main() {
    let mut x = 5;
    let mut y = 10;

    println!("Before swap: x = {} y = {}", x, y);

    let temp = x;
    x = y;
    y = temp;

    println!("After swap: x = {} y = {}", x, y);
}
