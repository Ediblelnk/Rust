#[allow(dead_code, unused_variables, unused_assignments)]

fn main() {
    variables_and_mutability();
}

fn variables_and_mutability() {
    /*
    by default, variables are immutable.
    */
    let x = 5;
    // x = 6; <-- this would cause an error!

    let mut y = 'c';
    println!("The value of y is: {y}");

    y = 'e'; // <-- this is fine, because of the identifier "mut"
    println!("The value of y is: {y}");

    // CONSTANTS

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // must declare type for a constant

    // SHADOWING

    let z = 5;
}
