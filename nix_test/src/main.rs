use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid, getppid};

fn main() {
    let pid = fork();

    match pid.expect("Fork Failed: Unable to create child process!") {
        Child => println!(
            "Hello from child process with pid: {} and parent pid:{}",
            getpid(),
            getppid()
        ),
        Parent { child } => {
            wait();
            println!(
                "Hello from parent process with pid: {} and child pid:{}",
                getpid(),
                child
            );
        }
    }
}
