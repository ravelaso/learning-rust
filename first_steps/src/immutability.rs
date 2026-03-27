
// A C# colleague claims their record is immutable. Translate this C# code to Rust and explain why Rust’s version is truly immutable:

// public record Config(string Host, int Port, List<string> AllowedOrigins);
// var config = new Config("localhost", 8080, new List<string> { "example.com" });
// // "Immutable" record... but:
// config.AllowedOrigins.Add("evil.com"); // Compiles! List is mutable.

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Config {
    host: String,
    port: u16,
    allowed_origins: Vec<String>,
}
impl Config {
    fn with_host(&self, host: impl Into<String>) -> Self{
        Config {
            host: host.into(),
            ..self.clone()
        }
    }
}

pub fn run(){

    let config = Config {
        host: "localhost".into(),
        port: 8080,
        allowed_origins: vec!["example.com".into()],
    };
    
    // config.allowed_origins.push("evil.com".into());
    // ❌ ERROR: cannot borrow `config.allowed_origins` as mutable
  
    let production = config.with_host("prod.example.com");
    println!("Dev: {:?}", config);       // original unchanged
    println!("Prod: {:?}", production);  // new copy with different host
}