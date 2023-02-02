fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    let mut arr = [4, 3, 2, 1];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
}