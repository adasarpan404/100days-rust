// Numeric literals can be type annotated by adding the type as a suffix.
// As an example, to specify that the literal 42 should have the type i32, write 42i32.
// The type of unsuffixed numeric literals will depend on how they are used.
// If no constraint exists, the compiler will use i32 for integers, and f64 for floating-point numbers.

fn main() {
    let x = 1u8;
    let y = 3u16;
    let z = 5f32;

    let i = 1;
    let f = 1.0;
    
    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
