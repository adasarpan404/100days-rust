fn main() {
    let array = [1, -3, 2, -5, 7, 6, -1, -4, 11, -23];
    let max_sum = max_subarray_sum(&array);

    println!("Maximum subarray sum: {}", max_sum);
}

fn max_subarray_sum(array: &[i32]) -> i32 {
    let mut max_sum = std::i32::MIN;
    let mut current_sum = 0;

    for &element in array {
        current_sum = current_sum.max(0) + element;
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}
