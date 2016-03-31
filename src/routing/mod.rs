use router::Router;
use super::index::createpoll::*;
use super::index::fileloader::*;
use super::index::frontpage::*;

pub fn setup_router() -> Router {
	let mut router = Router::new();
	router.get("/", frontpage);
	router.post("/createpoll", create_poll);
	router.get("/storage/:filename", file_handler);
	router
}

