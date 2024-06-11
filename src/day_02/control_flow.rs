#[allow(dead_code)]
pub fn control_flow_in_rust() {
    if_expressions();
    if_in_a_let_statement();
    types_of_loops_in_rust();
}

#[allow(dead_code)]
fn if_expressions() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

#[allow(dead_code)]
fn if_in_a_let_statement() {
    let age: i8 = 18;
    let can_vote = if age >= 18 { true } else { false };
    // the potential results from each arm must be same
    println!("can vote {can_vote}");
}

#[allow(dead_code)]
fn types_of_loops_in_rust() {
    loops_in_rust();
    while_loops_in_rust();
    for_loops_in_rust();
}

#[allow(dead_code)]
fn loops_in_rust() {
    // basic loop
    let mut counter = 1;
    loop {
        if counter > 30 {
            break;
        }
        println!("counter: {counter}");
        counter *= 2;
    }

    // returning values from loops
    let mut counter2 = 0;
    let result = loop {
        if counter2 > 15 {
            break counter2;
        }
        println!("counter2: {counter2}");
        counter2 += 5;
    };
    println!("result: {result}");

    /* If you have loops within loops, break and continue
    apply to the innermost loop at that point. You can
    optionally specify a loop label on a loop that you
    can then use with break or continue to specify that
    those keywords apply to the labeled loop instead of
    the innermost loop. Loop labels must begin with a single quote.
    */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

#[allow(dead_code)]
fn while_loops_in_rust() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        print!("{} ", a[index]);
        index += 1;
    }
    println!("");
}

#[allow(dead_code)]
fn for_loops_in_rust() {
    // looping through a container
    let a = [10, 20, 30, 40, 50];
    for element in a {
        print!("{element} ");
    }
    println!("");

    // looping in a range
    for i in 1..4 {
        print!("{i} ");
    }
    println!("");

    // looping in a range in reverse
    for i in (1..4).rev() {
        print!("{i} ");
    }
    println!("");

    // looping through a container index
    for i in 0..a.len() {
        print!("index {}: {} ", i, a[i]);
    }
    println!("");
}
