use std::time::Instant;

fn main() {
    const UPPER_BOUND: i32 = 1_000_000;
    let mut counter = 1;
    let mut current = 1;
    let mut local_bound: f64;
    let mut test: i32;

    let start = Instant::now();

    while current < UPPER_BOUND {
        current += 1;
        local_bound = f64::sqrt(current as f64) + 1.0;
        test = 2;

        while test < local_bound as i32 {
            if current % test == 0 {
                counter += 1;
                break;
            }
            test += 1;
        }
    }

    let duration = start.elapsed();

    println!(
        "There are {} prime numbers between 2 and {}",
        UPPER_BOUND - counter,
        UPPER_BOUND
    );
    println!("Calculations took {:?}", duration);
}
