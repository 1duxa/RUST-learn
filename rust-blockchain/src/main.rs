use cli::Cli;
use errors::Result;

mod errors;
mod block;
mod blockchain;
mod tests;
mod cli;
mod transaction;
mod tx;
mod wallet;
mod utxoset;
mod server;
const TARGET_HEXT: usize = 4;


fn main()  -> Result<()>{

    let mut cli = Cli::new().expect("Cli error");
    cli.run().expect("Error running cli");

    Ok(())
}
