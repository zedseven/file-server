//! Contains the route for serving the favicon of the server.

// Uses
use rocket::get;

use crate::constants::FAVICON_FILE_CONTENTS;

/// Serves the tool's favicon.
#[get("/favicon.ico")]
pub fn favicon_route() -> &'static [u8] {
	FAVICON_FILE_CONTENTS
}
