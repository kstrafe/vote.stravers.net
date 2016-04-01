use router::Router;
use super::index::createpoll::*;
use super::index::fileloader::*;
use super::index::frontpage::*;
use super::index::poll::*;
use super::index::vote::*;

pub fn setup_router() -> Router {
	let mut router = Router::new();
	router.get("/", frontpage);
	router.post("/createpoll", create_poll);
	router.get("/storage/:filename", file_handler);
	router.get("/vote/:value", see_poll);
	router.get("/votefor", see_poll);
	router.get("/:value", vote_handler);
	router
}

