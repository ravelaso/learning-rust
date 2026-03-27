pub struct Counter {
    value: i32, // Private by default
}

impl Counter {
    pub fn new() -> Counter {
        Counter { value: 0 }
    }
    pub fn increment(&mut self) {
        // &mut needed for mutation
        self.value += 1;
    }
    pub fn get_value(&self) -> i32 {
        self.value
    }
}


pub fn run(){
    let mut my_counter = Counter::new();
    println!("Count is: {} ", my_counter.get_value());
    for _  in 1..=5{
        my_counter.increment();
    }
    println!("Counter is now: {}", my_counter.get_value());
}