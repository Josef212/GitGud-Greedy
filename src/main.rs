// https://github.com/clap-rs/clap
// https://docs.rs/clap/latest/clap/
// https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html

use clap::Parser;

#[derive(Parser)]
#[clap(version="1.0", author="Josef212")]
struct Opts {
    #[clap(short, long, default_value="default.conf")]
    config: String,
    //#[clap(default_value="input.txt")]
    input: String,
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(version="1.0", author="Josef212")]
    Test(Test),
}

#[derive(Parser)]
struct Test {
    #[clap(short)]
    debug: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    
    println!("Value for config: {}", opts.config);
    println!("Using input file: {}", opts.input);
    
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be ridiculous"),
    }
    
    match opts.subcmd {
        Some(SubCommand::Test(t)) => {
            println!("Test debug info: {}", t.debug);
        }
        _ => {}
    }
}
