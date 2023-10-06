use rand::Rng;
use std::process::{Command, Stdio};
use std::{env, io, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => parent(),
        2 => child(),
        _ => eprintln!("Incorrect number of arguments. Expected 1 or 2."),
    }
}

fn parent() {
    // Generate a random number between 1 and 99 using the parent process's PID as the seed.
    let parent_pid = process::id() as u64;
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..100);

    println!("Parent sending to pipe: {}", random_number);

    // Spawn the child process
    let mut child = Command::new(env::current_exe().unwrap())
        .arg("child")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    // Send the random number to the child
    let child_stdin = child.stdin.as_mut().expect("Failed to open child's stdin");
    child_stdin
        .write_all(random_number.to_string().as_bytes())
        .expect("Failed to write to child's stdin");

    // Wait for a response from the child
    let mut response = String::new();
    let child_stdout = child
        .stdout
        .as_mut()
        .expect("Failed to open child's stdout");
    child_stdout
        .read_to_string(&mut response)
        .expect("Failed to read from child's stdout");

    println!("Child received from pipe: {}", response);

    if response.trim() == "Approved" {
        println!("Parent: Thanks for playing!");
    } else {
        println!("Parent: Wrong. Please play again!");
    }

    // Send "BYE" to the child
    child_stdin
        .write_all(b"BYE")
        .expect("Failed to write to child's stdin");

    // Wait for the child to finish
    child.wait().expect("Failed to wait for child process");
}

fn child() {
    // Read the random number from the parent
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from parent");
    let random_number: u32 = input.trim().parse().expect("Invalid input from parent");

    println!("Child received from pipe: {}", random_number);

    // Calculate the expected result
    let expected_result = random_number * process::id();

    // Send "Approved" or "Denied" to the parent based on the result
    if expected_result % 2 == 0 {
        println!("Child sending to pipe: Approved");
        println!("Child: Approved");
    } else {
        println!("Child sending to pipe: Denied");
        println!("Child: Denied");
    }

    // Wait for the "BYE" message from the parent
    let mut bye_message = String::new();
    io::stdin()
        .read_line(&mut bye_message)
        .expect("Failed to read from parent");

    println!("Child received from pipe: {}", bye_message);

    // Child process ends here
}
