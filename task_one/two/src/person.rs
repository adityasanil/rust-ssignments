pub struct Person {
    pub name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    pub fn person_details(&self) {
        println!("{} is {} years old.", self.name, self.age);
    }
}
