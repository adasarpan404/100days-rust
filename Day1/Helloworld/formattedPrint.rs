fn main(){
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);
    
    // Positional arguments can be used. Specifying an integer inside `{}`
    // {0} is being replaced by Alice
    // {1} is being replaced by Bob
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // I have used three arguments in it 
    // {0} will be replaced by Alice
    // {1} will be replaced by Bob
    // {2} will be replaced by John
    println!("{0} {1} {2} || {2} {0} {1} || {1} {0} {2} || {2} {1} {0}", "Alice", "Bob", "John");

    // String can be used as an arguments
    // {Subject} will be replaced by Subject="The quick brown fox"
    // {Verb} will be replaced by Verb="jumps over"
    // {Object} will be replaced by Object="the lazy dog"
    println!("{Subject} {Verb} {Object}", Object="the lazy dog", Verb="jumps over", Subject="The quick brown fox");

    // Different formatting can be invoked by specifying the format character after a
    // {} it will print as it is 
    // {:b} it will converted into binary
    // {:0} it will converted into octal
    // {:x} it will converted into hexadecimal
    // {:X} it will also converted into hexadecimal where every alphabet is converted to uppercase
    println!("Base 10:               {}",   69420); //69420
    println!("Base 2 (binary):       {:b}", 69420); //10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); //207454
    println!("Base 16 (hexadecimal): {:x}", 69420); //10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); //10F2C

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    
}