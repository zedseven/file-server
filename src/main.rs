use rocket::{self, config::Environment, Config};
use rocket_contrib::serve::StaticFiles;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("args: <path> [<port>]");
		return;
	}
	let static_path: &String = &args[1];
	let port: u16 = if args.len() >= 3 {
		match args[2].parse::<u16>() {
			Ok(p) => p,
			Err(e) => {
				eprintln!("The port value must be an integer: {}", e);
				return;
			}
		}
	} else {
		8080
	};

	let config = match Config::build(Environment::Development)
		.address("0.0.0.0")
		.port(port)
		.finalize()
	{
		Ok(c) => c,
		Err(e) => {
			eprintln!("The port value is invalid: {}", e);
			return;
		}
	};

	let launch_error = rocket::custom(config)
		.mount("/", StaticFiles::from(static_path))
		.launch();
	eprintln!("There was an error launching the server: {}", launch_error);
}
