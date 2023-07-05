use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

const UPPER_BOUND: usize = 1_000_000;
const LOWER_BOUND: usize = 0;

const NUM_THREADS: usize = 10;

struct Range {
    lower: usize,
    upper: usize,
}

fn main() {
    assert!((UPPER_BOUND - LOWER_BOUND) % NUM_THREADS == 0);

    let composites: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    let time = Instant::now();

    for i in 0..NUM_THREADS {
        let (thread_lower, thread_upper) =
            find_thread_range(&i, &LOWER_BOUND, &(UPPER_BOUND / NUM_THREADS));
        println!("{} {}", thread_lower, thread_upper);

        let composites = Arc::clone(&composites);
        let handle = thread::spawn(move || {
            let mut num = composites.lock().unwrap();

            let amount = find_composites(&Range {
                lower: thread_lower,
                upper: thread_upper,
            });

            *num += amount;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let primes: usize = UPPER_BOUND - *composites.lock().unwrap();

    let duration = time.elapsed();

    println!(
        "There are {} prime numbers between {} and {}",
        primes, LOWER_BOUND, UPPER_BOUND
    );
    println!("Calculations took {:?}", duration);
}

fn find_composites(range: &Range) -> usize {
    let mut value = range.lower;
    let mut composites = 0;

    let mut test: usize;

    while value < range.upper {
        value += 1;
        test = 2;

        let value_bound = f64::sqrt(value as f64) + 1.0;

        while test < value_bound as usize {
            if value % test == 0 {
                composites += 1;
                break;
            }
            test += 1;
        }
    }

    if range.lower == 0 {
        composites += 1;
    }

    composites
}

fn find_thread_range(i: &usize, start: &usize, step: &usize) -> (usize, usize) {
    (start + i * step, start + (i + 1) * step) // returns lower, upper
}
