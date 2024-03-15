mod person;
mod animal;

use A::B;
use A::{B, C};
use person::{Person as Animal};

fn main() {
    let mut person = Person::new("Alice".to_string(), 30);
    person.introduce();
    person.celebrate_birthday();

    let animal = animal::Animal::new("Buddy".to_string(), "Dog".to_string());
    animal.introduce();
    animal.celebrate_birthday();
}
