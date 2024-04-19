pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    pub fn introduce(&self) {
        println!("Hi, my name is {} and I'm {} years old.", self.name, self.age);
    }

    pub fn celebrate_birthday(&mut self) {
        self.age += 1;
        println!("Happy birthday! I'm now {} years old.", self.age);
    }
}
