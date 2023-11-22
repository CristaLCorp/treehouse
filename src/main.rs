use std::io::stdin;

fn what_is_your_name() -> String {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Cannot read line");
    user_input.trim().to_lowercase()
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNot { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Izok, welcome {}", self.name),
            VisitorAction::AcceptWithNot { note } => {
                println!("Welcome {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("No alcool to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in", self.name),
        }
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("jean", VisitorAction::Accept, 45),
        Visitor::new(
            "paul",
            VisitorAction::AcceptWithNot {
                note: String::from("Il est con, meffi"),
            },
            17,
        ),
        Visitor::new("cul", VisitorAction::Accept, 38),
        Visitor::new("jack", VisitorAction::Refuse, 98),
    ];

    loop {
        println!("Hello mofo, what is your name ?");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is the new guy, get on that list", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0))
                }
            }
        }
    }
    println!("The finale list of visitors : ");
    println!("{:#?}", visitor_list);
}
