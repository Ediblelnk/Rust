#[allow(dead_code)]
fn main() {
    variables_and_mutability();
    data_types();
}

fn variables_and_mutability() {
    /*
    by default, variables are immutable.
    */
    let _x = 5;
    // x = 6; <-- this would cause an error!

    let mut y = 'c';
    println!("The value of y is: {y}");

    y = 'e'; // <-- this is fine, because of the identifier "mut"
    println!("The value of y is: {y}");

    // CONSTANTS

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // must declare type for a constant
    println!("The Const: {THREE_HOURS_IN_SECONDS}");

    // SHADOWING

    let _z = 5;
    let _z = _z + 1;
    {
        let _z = _z * 2;
        println!("The value of z in the inner scope is: {_z}");
    }

    println!("The value of z is: {_z}");

    let spaces = String::from("    ");
    let spaces: u8 = spaces.len().try_into().unwrap();
    println!("The value of spaces is: {}", spaces);
}

fn data_types() {
    {
        let _guess: u32 = "42".parse().expect("NaN!");
    }
    /*
    A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
    */
    {
        //BOOLEAN TYPE
        let _b: bool = true;
    }
    {
        //NUMERIC TYPES - unsigned integers and their MAX
        let a: u8 = u8::MAX;
        let b: u16 = u16::MAX;
        let c: u32 = u32::MAX;
        let d: u64 = u64::MAX;
        let e: u128 = u128::MAX;
        println!(" a: {a} \n b: {b} \n c: {c} \n d: {d} \n e: {e}");

        //NUMERIC TYPES - signed integers and their MIN
        let f: i8 = i8::MIN;
        let g: i16 = i16::MIN;
        let h: i32 = i32::MIN;
        let i: i64 = i64::MIN;
        let j: i128 = i128::MIN;
        println!(" f: {f} \n g: {g} \n h: {h} \n i: {i} \n j: {j}");

        //NUMERIC TYPES - isize and usize
        let k: usize = usize::MAX;
        let l: isize = isize::MIN;
        println!(" k: {k} \n l: {l}");
    }
}
