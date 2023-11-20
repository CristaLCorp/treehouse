use std::io::stdin;

fn what_is_your_name() -> String {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Cannot read line");
    user_input.trim().to_lowercase()
}

fn main() {
    println!("Hello mofo, what is your name ?");
    let name = what_is_your_name();
    println!("hello {}", &name);
}
