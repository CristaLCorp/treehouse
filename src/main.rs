use std::io::stdin;

fn what_is_your_name() -> String {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Cannot read line");
    user_input.trim().to_lowercase()
}

fn can_get_in(name: &str) -> bool {
    let visitor_list = ["jean", "paul", "cul"];
    let mut can_get_in: bool = false;
    for visitor in &visitor_list {
        if visitor == &name {
            can_get_in = true;
        }
    }
    can_get_in
}

fn main() {
    println!("Hello mofo, what is your name ?");
    let name = what_is_your_name();

    if can_get_in(&name) {
        println!("you are in {} !", &name);
    } else {
        println!("fuck off you mofo !");
    }
}
