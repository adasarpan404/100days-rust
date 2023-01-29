// for pointers, destructuring is done by &, ref, and ref mut

// dereferencing is done by *

fn main() {
    let reference = &4;
    match reference {
        &val => println!("Got a value after destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value after dereferencing: {:?}", val),
    }

    let not_a_reference = 5;
    let ref a_reference = 5;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}
