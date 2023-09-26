pub struct Person {
    pub name: String,
    age: u32,
}

impl Person {
    pub fn person_create(name: String, age: u32) -> Person {
        Person { name, age }
    }

    pub fn person_details(&self) {
        println!("{} is {} years old.", self.name, self.age);
    }
}
