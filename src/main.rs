//! Doccumentation:
//! Tinny_HTTP: https://crates.io/crates/tiny_http
//! Clap: https://crates.io/crates/clap
//! TOML: https://crates.io/crates/toml

use clap::Parser;
use colorize::*;

#[path = "./server.rs"] mod server;

#[derive(Parser, Debug)]#[command(version, about, long_about = None)]/// Simple program to greet a person
struct Args {
    /// Root path
    #[arg(short, long)]
    root: String,

    /// Dedicated port
    #[arg(short, long)]
    port: u8,
}

fn main() -> Result<(), std::io::Error> {
    let _args = Args::parse();
    
    server::serve()?;
    /*let _ = server::serve().map_err(|error| {
        println!("{}", format!("[ERROR] {error}").red());
    });*/
    
    Ok(())
}
