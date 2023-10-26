use nix::sys::socket::*;

fn main() {
    match socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), None) {
        Ok(_) => println!("It worked!"),
        Err(why) => println!("DID NOT WORK: {}", why),
    }
}
