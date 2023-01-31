fn main() {
    let rows = 10;

    for i in 0..rows {
        for j in 0..=i {
            print!("{} ", pascal(i, j));
        }
        println!();
    }
}

fn pascal(i: u32, j: u32) -> u32 {
    if j == 0 || i == j {
        return 1;
    }
    pascal(i - 1, j - 1) + pascal(i - 1, j)
}
