pub trait Action {
    fn say(&self);
}

pub struct Person {
    pub name: String,
}

impl Action for Person {
    fn say(&self){
        println!("Hello, {}", self.name);
    }
}

fn main() {
    let person = Person {
        name: String::from("max"),
    };
    person.say();
}
