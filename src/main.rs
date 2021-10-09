mod cli;
mod favicon;

use rocket::{self, fs::FileServer, main as rocket_main, routes, Config};

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
	if let Err(launch_error) = rocket::custom(config)
		.mount("/", routes![favicon_route])
		.mount("/", FileServer::from(static_path))
		.launch()
		.await
	{
		eprintln!("There was an error launching the server: {}", launch_error);
	}
}
