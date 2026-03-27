mod console_input;
mod cli_arguments;
mod clap_example;

fn main(){
    console_input::run();
    cli_arguments::run();
    clap_example::run();
}