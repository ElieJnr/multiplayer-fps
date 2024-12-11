use std::io::{self, Write};

fn get_user_input(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn init_connection() {
    let _ip_address = get_user_input("Enter IP Address: ");
    let _username = get_user_input("Enter Name: ");
    println!("Starting... ")
}
