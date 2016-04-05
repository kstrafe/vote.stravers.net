use iron::prelude::*;
use mount::Mount;
use dbcon::DbCon;
use response;
use controllers;
use oven;
use std::path::Path;
use staticfile::Static;

pub fn get_middleware() -> Mount {
	info!("Setting up the middleware chain...");
	let mut chain = Chain::new(controllers::index::index);
	chain.link_before(response::ResponseTime);
	chain.link(oven::new(vec![]));
	chain.link_before(DbCon);
	chain.link_after(response::ResponseTime);

	let mut mount = Mount::new();
	mount.mount("/file/", Static::new(Path::new("src/")));
	mount.mount("/", chain);
	mount
}
