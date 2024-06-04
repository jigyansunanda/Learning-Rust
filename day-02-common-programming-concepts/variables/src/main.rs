fn main() {
    // variables and mutability:
    // can reassign values to
    // variables only if they are mutable
    let mut x = 5;
    println!("x is {x}");
    x = 10;
    println!("x is now {x}");

    // can't use mut with constants
    // the type of the value must be annotated
    const PRICE: u64 = 23454353434253454;
    println!("price is {PRICE}");

    // Shadowing with block scope
    let y = 5;
    let y = y + 1;
    {
        // this y declaration shadows
        // the value only here inside
        // this block scope
        let y = y * 2;
        println!("y is {y}");
    }
    println!("y is {y}");

    // using "let" keyword, effectively
    // creating a new variable.
    let spaces = "   ";
    println!("spaces: {spaces}");
    let spaces = spaces.len();
    println!("spaces: {spaces}");
    // but declaring with "mut"
    // will create a compilation error.
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // this will create a compilation error.
}
