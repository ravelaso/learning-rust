#[derive(Debug)]

pub struct Person {
    pub name: String,
    pub age: u32,
    hobbies: Vec<String>,
}
#[allow(dead_code)]
impl Person {
    pub fn new(name: String, age: u32) -> Person {
        Person {
            name,
            age,
            hobbies: Vec::new(),
        }
    }
    pub fn add_hobby(&mut self, hobby: String) {
        self.hobbies.push(hobby);
    }

    pub fn get_info(&self) -> String {
        format!("{} is {}, years old", self.name, self.age)
    }

    pub fn hobbies(&self) -> &Vec<String> {
        &self.hobbies
    }

    /// &self - Immutable borrow (most common)
    /// Use when you only need to read the data
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// &mut self - Mutable borrow
    /// Use when you need to modify the data
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// self - Take ownership (less common)
    /// Use when you want to consume the struct
    pub fn consume(self) -> String {
        self.name // Person is moved, no longer accessible
    }
}

pub fn person_test() {
    // Rust struct creation and usage
    let mut person = Person::new("Ravelo".to_string(), 30);
    person.add_hobby("Music".to_string());
    person.add_hobby("Photography".to_string());

    println!("{}", person.get_info());
    println!("Hobbies: {:?}", person.hobbies());

    // Modifiy public fields directly
    person.age = 31;

    // Debug print entire struct
    println!("{:?}", person)
}

#[derive(Debug)]
pub struct Calculator {
    memory: i32,
}

impl Calculator {
    // Associated function (like static method) - no self parameter
    pub fn new() -> Calculator {
        Calculator { memory: 0 }
    }

    // Associated function with parameters
    pub fn with_memory(initial_memory: i32) -> Calculator {
        Calculator {
            memory: initial_memory,
        }
    }

    // Method that borrows immutably (&self)
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    // Method that borrows mutably (&mut self)
    pub fn store_in_memory(&mut self, value: i32) {
        self.memory = value;
    }

    // Method that takes ownership (self)
    pub fn into_memory(self) -> i32 {
        self.memory // Calculator is consumed
    }

    // Getter method
    pub fn memory(&self) -> i32 {
        self.memory
    }
}

pub fn calculator_test() {
    // Associated functions called with ::
    let mut calc = Calculator::new();
    let _calc2 = Calculator::with_memory(42);

    // Methods called with .
    let result = calc.add(5, 3);
    calc.store_in_memory(result);

    println!("Memory: {}", calc.memory());

    // Consuming method
    let memory_value = calc.into_memory(); // calc is no longer usable
    println!("Final memory: {}", memory_value);
}
