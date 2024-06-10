#[allow(dead_code)]
pub fn compound_data_types_in_rust() {
    println!("------------ Tuples ------------");
    tuple_data_type_in_rust();
    println!("\n");
    println!("------------ Arrays ------------");
    array_data_type_in_rust();
}

// Tuples
#[allow(dead_code)]
fn tuple_data_type_in_rust() {
    /*  A tuple is for grouping together values
    with a variety of types into one compound type.
    Tuples have a fixed length. i.e. once declared,
    they cannot grow or shrink in size.
    */
    let tup: (i32, f64, u8, bool) = (500, 6.4, 1, true);
    println!("tup: {:?}", tup);

    // destructuring a tuple by pattern matching
    let (a, b, c, d) = tup;
    println!("a: {a}, b: {b}, c: {c}, d:{d}");

    // accessing by index of values using "."
    println!(
        "index 0: {}, index 1: {}, index 2: {}, index 3: {}",
        tup.0, tup.1, tup.2, tup.3
    );

    // however below line will result in error.
    // as The {tup.0} syntax is not directly supported
    // within "println!"" macros for accessing tuple elements
    // println!("index 0: {tup.0}, index 1: {tup.1}, index 2: {tup.2}, index 3:{tup.3}");
}

// Arrays
#[allow(dead_code)]
fn array_data_type_in_rust() {
    // Every element of Array should have same type.
    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    // To create an array of specific type:
    // let array_name: [type; size] = [...];
    let b: [bool; 5] = [true, false, false, true, false];
    println!("b: {:?}", b);

    // To create an array with some value:
    // let array_name = [value; size];
    let c = [3; 10];
    println!("c: {:?}", c);

    // Accessing array element by index
    println!("element at index 7: {}", c[7]);

    // Size of array
    println!("size of array c: {}", c.len());
}
