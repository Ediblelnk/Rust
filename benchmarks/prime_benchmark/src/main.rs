use std::time::Instant;

const UPPER_BOUND: usize = 1_000_000_000;
const LOWER_BOUND: usize = 2;

fn main() {
    let start = Instant::now();

    let primes = UPPER_BOUND
        - find_composites(&Range {
            lower: LOWER_BOUND,
            upper: UPPER_BOUND,
        });

    let duration = start.elapsed();

    println!(
        "There are {} prime numbers between {} and {}",
        primes, LOWER_BOUND, UPPER_BOUND
    );
    println!("Calculations took {:?}", duration);
}

struct Range {
    lower: usize,
    upper: usize,
}

fn find_composites(range: &Range) -> usize {
    let mut value = range.lower;
    let mut composites = 1;

    let mut test: usize;

    while value < range.upper {
        value += 1;
        let value_bound = f64::sqrt(value as f64) + 1.0;

        test = 2;

        while test < value_bound as usize {
            if value % test == 0 {
                composites += 1;
                break;
            }
            test += 1;
        }
    }

    composites
}
