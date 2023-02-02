fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 1..len-i {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
    }
}

fn main() {
    let mut arr = [4, 3, 2, 1];
    bubble_sort(&mut arr);
    println!("{:?}", arr);
}