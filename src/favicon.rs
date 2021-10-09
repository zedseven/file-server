use rocket::get;

const FAVICON: &[u8] = include_bytes!("../favicon.ico");

#[get("/favicon.ico")]
pub fn favicon_route() -> &'static [u8] {
	FAVICON
}
