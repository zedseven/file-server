use clap::{App, Arg, ArgMatches};
use rocket::{self, config::Environment, Config};
use rocket_contrib::serve::StaticFiles;
use std::env;

fn main() {
	let matches: ArgMatches = App::new("File Server")
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about("A basic file server for serving up static files over HTTP.")
		.arg(
			Arg::new("input")
				.required(true)
				.index(1)
				.about("The directory to serve"),
		)
		.arg(
			Arg::new("port")
				.short('p')
				.long("port")
				.takes_value(true)
				.default_value("8080")
				.allow_hyphen_values(true)
				.validator(|s| match s.parse::<u16>() {
					Ok(_) => Ok(()),
					Err(_) => Err(String::from("must be parsable as u16")),
				})
				.about("Sets the port to host the server on"),
		)
		.get_matches();

	let static_path: String = String::from(matches.value_of("input").unwrap());
	let port: u16 = matches.value_of("port").unwrap().parse::<u16>().unwrap();

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
