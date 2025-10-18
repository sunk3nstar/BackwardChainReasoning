use clap::Parser;
use reasoning::cli::{Cli, prove};

fn main() {
    let args = Cli::parse();
    prove(&args).unwrap();
}
