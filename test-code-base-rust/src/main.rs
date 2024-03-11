mod person;
mod animal;

fn main() {
    let mut person = person::Person::new("Alice".to_string(), 30);
    person.introduce();
    person.celebrate_birthday();

    let animal = animal::Animal::new("Buddy".to_string(), "Dog".to_string());
    animal.introduce();
    animal.celebrate_birthday();
}
