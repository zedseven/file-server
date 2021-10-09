//! A basic file server made with [Rocket](https://rocket.rs/) for serving up static files over HTTP.

// Linting rules
#![warn(
	missing_crate_level_docs,
	missing_docs,
	clippy::complexity,
	clippy::correctness,
	clippy::perf,
	clippy::style,
	clippy::suspicious,
	clippy::pedantic,
	clippy::filetype_is_file,
	clippy::str_to_string
)]
#![allow(
	dead_code,
	unused_macros,
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	clippy::cast_precision_loss,
	clippy::cast_sign_loss,
	clippy::doc_markdown,
	clippy::module_name_repetitions,
	clippy::similar_names,
	clippy::too_many_lines,
	clippy::unnecessary_wraps
)]

mod cli;
mod favicon;

use rocket::{self, custom, fs::FileServer, main as rocket_main, routes, Config};

use crate::{cli::parse_cli_arguments, favicon::favicon_route};

#[rocket_main]
async fn main() {
	// Parse CLI
	let matches = parse_cli_arguments();
	let static_path = String::from(matches.value_of("input").unwrap());
	let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();

	// Configure
	let mut config = Config::debug_default();
	config.address = "0.0.0.0".parse().unwrap();
	config.port = port;

	// Launch
	if let Err(launch_error) = custom(config)
		.mount("/", routes![favicon_route])
		.mount("/", FileServer::from(static_path))
		.launch()
		.await
	{
		eprintln!("There was an error launching the server: {}", launch_error);
	}
}
