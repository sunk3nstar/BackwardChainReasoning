use clap::Parser;
use reasoning::{
    ReasoningError,
    cli::{Cli, prove},
};

fn main() -> Result<(), ReasoningError> {
    let args = Cli::parse();
    let provement = prove(&args);
    if let Err(e) = provement {
        match e {
            ReasoningError::UnifyError | ReasoningError::ThetaError => {
                println!("无法证明命题为真");
            }
            _ => {
                eprintln!("{e}");
                return Err(e);
            }
        }
    } else {
        println!("命题为真");
    }
    Ok(())
}
