#[derive(Debug)]
enum Command {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    Count(u32),
}

fn parse_command(input: &str) -> Result<Command, String> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    match parts[0] {
        "quit" => Ok(Command::Quit),
        "echo" => {
            let msg = parts.get(1).unwrap_or(&"").to_string();
            Ok(Command::Echo(msg))
        }
        "move" => {
            let args = parts.get(1).ok_or("move requires 'x y'")?;
            let coords: Vec<&str> = args.split_whitespace().collect();
            let x = coords
                .get(0)
                .ok_or("missing x")?
                .parse::<i32>()
                .map_err(|e| e.to_string())?;
            let y = coords
                .get(1)
                .ok_or("missing y")?
                .parse::<i32>()
                .map_err(|e| e.to_string())?;
            Ok(Command::Move { x, y })
        }
        "count" => {
            let n = parts
                .get(1)
                .ok_or("count requires a number")?
                .parse::<u32>()
                .map_err(|e| e.to_string())?;
            Ok(Command::Count(n))
        }
        other => Err(format!("Unknown command: {other}")),
    }
}

fn execute(cmd: &Command) -> String {
    match cmd {
        Command::Quit => "Goodbye!".to_string(),
        Command::Echo(msg) => msg.clone(),
        Command::Move { x, y } => format!("Moved to ({}, {})", x, y),
        Command::Count(n) => format!("Counted to {}", n),
    }
}

pub fn test() {
    println!("Testing command echo");
    let input = "echo Hello, world!";
    let cmd = parse_command(input).unwrap();
    println!("{}", execute(&cmd));

    println!("Testing command move");
    let input = "move 10 20";
    let cmd = parse_command(input).unwrap();
    println!("{}", execute(&cmd));

    println!("Testing command count");
    let input = "count 100";
    let cmd = parse_command(input).unwrap();
    println!("{}", execute(&cmd));
}
