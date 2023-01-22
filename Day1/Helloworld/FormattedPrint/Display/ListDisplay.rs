use std::fmt; // importing fmt module

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Extract the value using tuple indexing,
//         // and create a reference to `vec`.

//         let vec = &self.0;

//         // Iterate over `v` in `vec` while enumerating the iteration
//         // count in `count`.

//         write!(f, "[")?;

//         for (count, v) in vec.iter().enumerate() {
//             // For every element except the first, add a comma.
//             // Use the ? operator to return on errors.
//             if count != 0 {
//                 write!(f, ", ")?;
//             }

//             write!(f, "{}", v)?;
//         }
//         // closing the opening bracket and returning the result
//         write!(f, "]")
//     }
// }

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: ", count)?;
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    // [0: 1, 1: 2, 2: 3]
}
