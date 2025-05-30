use rackit::{Cli, execute_command, Result};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    execute_command(cli.command, cli.verbose, cli.quiet)
}