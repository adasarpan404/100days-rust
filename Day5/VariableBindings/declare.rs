// It's possible to declare variable bindings first, and initialize them later.
// However, this form is seldom used, as it may lead to the use of uninitialized variables.

fn main() {
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // println!("another binding: {}", another_binding);
    // Error will be thrown due to we are using unintialized variable
    another_binding = 1;

    println!("another binding: {}", another_binding);
}
