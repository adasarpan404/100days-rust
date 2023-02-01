fn main() {
    let mut arr = [64, 25, 12, 22, 11];
    selection_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..n {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }

        arr.swap(i, min_idx);
    }
}
