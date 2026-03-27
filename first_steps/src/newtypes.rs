/// A validated email address — the type itself IS the proof of validity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    /// The ONLY way to create an Email — validation happens once at construction.
    pub fn new(raw: &str) -> Result<Self, &'static str> {
        if raw.contains('@') && raw.len() > 3 {
            Ok(Email(raw.to_lowercase()))
        } else {
            Err("invalid email format")
        }
    }

    /// Safe access to the inner value
    pub fn as_str(&self) -> &str { &self.0 }
}

/// A validated age — impossible to create an invalid one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Age(u8);

impl Age {
    pub fn new(raw: u8) -> Result<Self, &'static str> {
        if raw <= 150 { Ok(Age(raw)) } else { Err("age out of range") }
    }
    pub fn value(&self) -> u8 { self.0 }
}

// Now functions take PROVEN types — no re-validation needed!
fn create_user(email: Email, age: Age) -> User {
    // email is GUARANTEED valid — it's a type invariant
    User { email, age }
}

fn send_email(to: &Email) {
    // No validation needed — Email type proves validity
    println!("Sending to: {}", to.as_str());
}