// Constructor Patterns
#[derive(Debug)]
#[allow(dead_code)]
pub struct Configuration {
    pub database_url: String,
    pub max_connections: u32,
    pub enable_logging: bool,
}
#[allow(dead_code)]
impl Configuration {
    // Default constructor
    pub fn new() -> Configuration {
        Configuration {
            database_url: "localhost".to_string(),
            max_connections: 10,
            enable_logging: false,
        }
    }
    // Parameterized constructor
    pub fn with_database(database_url: String, max_connections: u32) -> Configuration {
        Configuration {
            database_url,
            max_connections,
            enable_logging: false,
        }
    }
    // Factory method
    pub fn for_production() -> Configuration {
        Configuration {
            database_url: "prod.db.server".to_string(),
            max_connections: 100,
            enable_logging: true,
        }
    }

    // Builder pattern method
    pub fn enable_logging(mut self) -> Configuration {
        self.enable_logging = true;
        self // Return self for chaining
    }

    pub fn max_connections(mut self, count: u32) -> Configuration {
        self.max_connections = count;
        self
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused)]
pub fn configuration_test() {
    // Different construction patterns
    let config1 = Configuration::new();
    let config2 = Configuration::with_database("localhost:5432".to_string(), 20);
    let config3 = Configuration::for_production();

    // Builder pattern
    let config4 = Configuration::new().enable_logging().max_connections(50);

    // Using Default trait
    let config5 = Configuration::default();

    println!("{:?}", config4);
}
