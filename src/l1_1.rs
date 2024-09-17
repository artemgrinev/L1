trait Action {
    fn say(&self);
}

struct Person {
    name: String,
}

impl Action for Person {
    fn say(&self) {
        println!("Helow {}", self.name);
    }
}

fn main() {
    let artem = Person{ name: String::from("Artem") };
    artem.say()
}