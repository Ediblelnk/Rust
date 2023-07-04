use std::time::Instant;

const UPPER_BOUND: usize = 1_000_000;
const LOWER_BOUND: usize = 2;

fn main() {
    let start = Instant::now();

    let mut prime_list: Vec<usize> = vec![2];

    find_primes(
        &Range {
            lower: LOWER_BOUND,
            upper: UPPER_BOUND,
        },
        &mut prime_list,
    );

    let duration = start.elapsed();

    println!(
        "There are {} prime numbers between {} and {}",
        prime_list.len(),
        LOWER_BOUND,
        UPPER_BOUND
    );
    println!("Calculations took {:?}", duration);
}

struct Range {
    lower: usize,
    upper: usize,
}

fn find_primes(range: &Range, prime_list: &mut Vec<usize>) -> () {
    let lower_bound: usize = match range.lower {
        0 | 1 => 2,
        _ => range.lower.clone(),
    };

    let mut is_prime: bool;
    for num in lower_bound..range.upper {
        is_prime = true;
        for prime in &mut *prime_list {
            if num % *prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            prime_list.push(num);
        }
    }
}
