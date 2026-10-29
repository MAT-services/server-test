use std::path::{/*Path, */PathBuf};
use colorize::*;
use tiny_http::{Server, Response};

pub fn serve(root: PathBuf, port: u128) -> Result<(), std::io::Error> {
	let _info = "[INFO]".blue();
	let _error = "[ERROR]".red();

	let server = Server::http(format!("0.0.0.0:{port}")).unwrap();
	let cfgfile = format!("{}/.serverenv/config.toml", root.to_str().unwrap());
	println!("Start serving on \"{}\" with port \"{port}\".\nConfigFile at \"{cfgfile}\".", root.to_str().unwrap());

	for request in server.incoming_requests() {

		let relative = request.url().trim_start_matches('/');

		println!("{_info} Requested: {}", request.url());

		let filepath = match root.join(relative).canonicalize() {
			Ok(path) => path,
			Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
				println!("{_error} 404: Not found");
				continue;
			}
			Err(err) => return Err(err),
		};

		if !filepath.starts_with(&root) {
			println!("{_error} 403: Access denied (out of root)");
			continue;
		}
		
		request.respond(Response::from_string("test"))?;
	}
	Ok(())
}