use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid, getppid, Pid};

fn child() {
    println!(
        "Hello from child process with pid: {} and parent pid:{}",
        getpid(),
        getppid()
    )
}

fn parent(child: Pid) {
    let _ = wait();
    println!(
        "Hello from parent process with pid: {} and child pid:{}",
        getpid(),
        child
    );
}

fn main() {
    let (mut child_reader, mut child_writer) = os_pipe::pipe().unwrap();
    let (mut parent_reader, mut parent_writer) = os_pipe::pipe().unwrap();

    match unsafe { fork() }.expect("Fork Failed: Unable to create child process!") {
        Child => child(),
        Parent { child } => parent(child),
    }
}
