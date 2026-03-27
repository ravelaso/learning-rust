use clap::Parser;

/// A simple file processor — this doc comment becomes the help text
#[derive(Parser, Debug)]
#[command(name = "processor", version, about)]
struct Args {
    /// Input file to process
    #[arg(short, long)]
    input: String,

    /// Output file (defaults to stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// Enable verbose logging
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Number of worker threads
    #[arg(short = 'j', long, default_value_t = 4)]
    threads: usize,
}

pub fn run() {
    let args = Args::parse();
    if args.verbose {
        println!("Input:   {}", args.input);
        println!("Output:  {:?}", args.output);
        println!("Threads: {}", args.threads);
    }
    // Use args.input, args.output, etc.
}
