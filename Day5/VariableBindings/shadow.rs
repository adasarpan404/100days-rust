fn main() {
    let shadow_binding = 1;
    {
        println!("before being shadowed: {}", shadow_binding);

        let shadow_binding = "abc";

        println!("shadowed in inner block: {}", shadow_binding);
    }
    println!("outside inner block: {}", shadow_binding);

    let shadow_binding = 2;

    println!("shadowed in outer block: {}", shadow_binding);
}
