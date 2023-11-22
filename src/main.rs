use std::io::stdin;

fn what_is_your_name() -> String {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Cannot read line");
    user_input.trim().to_lowercase()
}

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}


fn main() {
    println!("Hello mofo, what is your name ?");
    let name = what_is_your_name();

    let visitor_list = [
        Visitor::new("jean", "jean est un con"),
        Visitor::new("paul", "paul est un con"),
        Visitor::new("cul", "cul est ok"),
    ];

    let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("Sorry, not on the damn list ! getoutaheer !")
    }

}
