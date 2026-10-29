//! Doccumentation:
//! Tinny_HTTP: https://crates.io/crates/tiny_http
//! Clap: https://crates.io/crates/clap
//! TOML: https://crates.io/crates/toml

use clap::Parser;

#[path = "./server.rs"] mod server;

#[derive(Parser, Debug)]#[command(version, about, long_about = None)]/// Simple program to greet a person
struct Args {
	/// Root path
	#[arg(short, long)]
	root: String,

	/// Dedicated port
	#[arg(short, long)]
	port: u128,
}

fn main() -> Result<(), std::io::Error> {
	let args = Args::parse();
	
	server::serve(std::path::Path::new(&args.root).canonicalize()?, args.port)?;
	/*let _ = server::serve().map_err(|error| {
		println!("{}", format!("[ERROR] {error}").red());
	});*/
	
	Ok(())
}
