#[derive(Debug, PartialEq)]
pub enum PhoneType {
    Mobile,
    Home,
    Work,
}
#[derive(Debug)]
pub struct Contact {
    name: String,
    phones: Vec<(PhoneType, String)>,
}

impl Contact {
    pub fn new(name: impl Into<String>) -> Self {
        Contact {
            name: name.into(),
            phones: Vec::new(),
        }
    }
    pub fn add_phone(&mut self, kind: PhoneType, number: impl Into<String>) {
        self.phones.push((kind, number.into()));
    }

    pub fn mobile_numbers(&self) -> Vec<&str> {
        self.phones
            .iter()
            .filter(|(kind, _)| *kind == PhoneType::Mobile)
            .map(|(_, num)| num.as_str())
            .collect()
    }
}

pub fn test() {
    let mut alice = Contact::new("Ravelo");
    alice.add_phone(PhoneType::Mobile, "+1-555-0100");
    alice.add_phone(PhoneType::Work, "+1-555-0200");
    alice.add_phone(PhoneType::Mobile, "+1-555-0101");
    alice.add_phone(PhoneType::Home, "+1-669-0420");

    println!(
        "{}'s mobile numbers: {:?}",
        alice.name,
        alice.mobile_numbers()
    );
}
