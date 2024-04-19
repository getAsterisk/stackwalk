pub struct Animal {
    name: String,
    species: String,
}

impl Animal {
    pub fn new(name: String, species: String) -> Self {
        Animal { name, species }
    }

    pub fn introduce(&self) {
        println!("Hi, my name is {} and I'm a {}.", self.name, self.species);
    }

    pub fn celebrate_birthday(&self) {
        println!("{} is celebrating another year!", self.name);
    }
}
