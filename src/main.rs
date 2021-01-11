use rocket;
use rocket_contrib::serve::StaticFiles;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("args: <path>");
		return;
	}
	let static_path = &args[1];

	rocket::ignite()
		.mount("/", StaticFiles::from(static_path))
		.launch();
}
