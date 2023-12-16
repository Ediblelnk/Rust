use salting as lib;

fn main() {
    let mut user = lib::User::new("Peter");

    user.set_password("password");
    match user.check_password("password1") {
        Ok(()) => println!("Password was accepted!"),
        Err(e) => println!("Password was not accepted: {:?}", e),
    }

    user.log_to_file("users.dat");
}
