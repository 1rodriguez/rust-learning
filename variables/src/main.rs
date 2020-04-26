fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let _spaces = "   ";
    let _spaces = _spaces.len(); // properly shadow, essentially a new variable with its own type

//    let mut characters = "cccc";
//    characters = characters.len(); // not allowed, not proper shadowing
    // the type of a mutable variable can't be changed
}
