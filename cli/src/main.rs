use std::io;

pub struct User {
    username: String,
    password: String,
    secret: String,
}

type Users = Vec<User>;

fn read_from_stdin(label: &str) -> String {
    let mut buffer = String::new();
    println!("{}", label);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read from stdin");
    buffer.trim().to_owned()
}

fn main() {
    let mut users: Users = Vec::new();

    println!("Xharlet-cli");

    loop {
        let input = read_from_stdin("Select an option:\n1. Register\n0. Exit\nEnter your choice: ");
        match input.as_str() {
            "1" => {
                let username = read_from_stdin("Enter username: ");
                let password = read_from_stdin("Enter password: ");
                let secret = read_from_stdin("Enter secret: ");
                let user = User {
                    username,
                    password,
                    secret,
                };
                users.push(user);
            }
            "0" => {
                println!("Goodbye");
                break;
            }
            "accounts" => {
                for user in &users {
                    println!("Username: {}", user.username);
                }
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
