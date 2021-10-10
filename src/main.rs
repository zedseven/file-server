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

// Modules
mod cli;
mod favicon;

// Uses
use std::net::{IpAddr, Ipv4Addr};

use listing_file_server::ListingFileServer;
use rocket::{self, custom, fs::Options, main as rocket_main, routes, Config};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{cli::parse_cli_arguments, favicon::favicon_route};

// Constants
const LIST_TEMPLATE_FILE: &str = include_str!("../static/list.html.tera");

/// The context for a directory listing, containing the directory path and
/// contents.
#[derive(Serialize)]
struct ListContext {
	directory: String,
	entry_list: Vec<String>,
}

/// Main.
#[rocket_main]
async fn main() {
	// Parse CLI
	let matches = parse_cli_arguments();
	let static_path = String::from(matches.value_of("input").unwrap());
	let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();

	// Configure
	let config = Config::figment()
		.merge(("address", IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))))
		.merge(("port", port))
		// A hacky solution to allow launch without an actual template dir (https://github.com/SergioBenitez/Rocket/issues/1792#issuecomment-908900295)
		.merge(("template_dir", "."));

	// Launch
	if let Err(launch_error) = custom(config)
		// Attach the template engine for directory listing
		.attach(Template::custom(|engines| {
			engines
				.tera
				.add_raw_template("list.html", LIST_TEMPLATE_FILE)
				.expect("Unable to add the directory listing template")
		}))
		// Favicon route
		.mount("/", routes![favicon_route])
		// File-serving route
		.mount(
			"/",
			ListingFileServer::new(
				static_path,
				Options::DotFiles | Options::NormalizeDirs,
				|directory, mut entry_list| {
					if directory != "/" {
						entry_list.insert(0, String::from(".."));
					}
					Template::render(
						"list.html",
						ListContext {
							directory,
							entry_list,
						},
					)
				},
			),
		)
		// Launch
		.launch()
		.await
	{
		eprintln!("There was an error launching the server: {}", launch_error);
	}
}
