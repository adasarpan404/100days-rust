// variable binding are immutable by default in rust
// This can be overridden by mut keywords

fn main(){
    let _immutable_binding = 1;
    let mut _mutable_binding = 1;

    println!("Before mutation: {}", _mutable_binding);

    _mutable_binding += 1;

    println!("After mutation: {}", _mutable_binding);

    // _immutable_binding+=1
    // Error will occur because variable binding is immutable by default.
}