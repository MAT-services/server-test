use clap::Parser;

use crate::server::serve;
#[path = "./server.rs"] mod server;

#[derive(Parser, Debug)]#[command(version, about, long_about = None)]/// Simple program to greet a person
struct Args {
    /// Root path
    #[arg(short, long)]
    path: String,

    /// Dedicated port
    #[arg(short, long)]
    port: u8,
}

fn main() -> Result<(), std::io::Error> {
    let _args = Args::parse();
    
    serve();
    
    Ok(())
    
}
