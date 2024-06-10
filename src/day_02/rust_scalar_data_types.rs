#[allow(dead_code)]
pub fn data_types_in_rust() {
    /* A scalar type represents a single value.
    Rust has four primary scalar types:
        1. Integers
        2. Floating Point Numbers
        3. Booleans
        4. Characters
    */

    // integers
    let a: i8 = 127;
    let b: i16 = 32767;
    let c: i32 = 243533243;
    let d: i64 = 34534635464452343;
    let e: i128 = 2352345354654645645;
    println!("a:{a}, b:{b}, c:{c}, d:{d}, e{e}");

    // floating point numbers
    let f: f32 = 21.0003380;
    let g: f64 = 6876574.8743278640;
    println!("f:{f}, g:{g}");

    // boolean values
    let i: bool = true;
    let j: bool = false;
    println!("i:{i}, j:{j}");

    // characters
    let k: char = 'k';
    let l: char = 'L';
    println!("k:{k}, l:{l}");

    /* Compound types can group multiple values into one type.
    Rust has two primitive compound types:
        1. Tuples
        2. Arrays
    */
}
