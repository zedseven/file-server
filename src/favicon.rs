// Uses
use rocket::get;

// Constants
const FAVICON: &[u8] = include_bytes!("../static/favicon.ico");

/// Serves the tool's favicon.
#[get("/favicon.ico")]
pub fn favicon_route() -> &'static [u8] {
	FAVICON
}
