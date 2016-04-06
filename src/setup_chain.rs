use iron::prelude::*;
use mount::Mount;
use controllers;
use oven;
use std::path::Path;
use staticfile::Static;
use middleware::*;

pub fn get_middleware() -> Mount {
	info!("Setting up the middleware chain...");
	let mut chain = Chain::new(controllers::index::index);
	chain.link_before(ResponseTime);
	chain.link(oven::new(vec![]));
	chain.link_before(DbCon);
	chain.link_after(Html);
	chain.link_after(ResponseTime);

	info!("Setting up the mounds");
	let mut mount = Mount::new();
	mount.mount("/file/", Static::new(Path::new("src/")));
	mount.mount("/", chain);
	mount
}
