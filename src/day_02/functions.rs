#[allow(dead_code)]
pub fn functions_in_rust() {
    expressions();
    println!("are we returning values? {}", function_with_return_values());
    println!("a + b = {}", function_with_input_params(10, 20));
}

#[allow(dead_code)]
fn expressions() {
    /* Note that the x + 1 line doesn’t have a semicolon
    at the end. Expressions do not include ending semicolons.
    If you add a semicolon to the end of an expression,
    you turn it into a statement, and it won't return a value.
    */
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

#[allow(dead_code)]
fn function_with_return_values() -> i32 {
    /* You can return early from a function
    by using the return keyword and specifying
    a value, but most functions return
    the last expression implicitly.

    Note that the expression does not have a
    semicolon ";" at the end.
    */
    5
}

#[allow(dead_code)]
fn function_with_input_params(a: i32, b: i32) -> i32 {
    a + b
    // return a + b;
}
