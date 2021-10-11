//! Defines and parses the CLI arguments.

// Uses
use clap::{App, Arg, ArgMatches};

/// Defines the CLI arguments and parses user input.
pub fn parse_cli_arguments() -> ArgMatches {
	App::new("File Server")
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about("A basic file server for serving up static files over HTTP.")
		.arg(
			Arg::new("input")
				.required(true)
				.about("The directory to serve files from"),
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
		.get_matches()
}
