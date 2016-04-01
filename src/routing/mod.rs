use router::Router;
use super::index::createpoll::*;
use super::index::fileloader::*;
use super::index::frontpage::*;
use super::index::stats::*;
use super::index::poll::*;
use super::index::vote::*;

pub fn setup_router() -> Router {
	let mut router = Router::new();
	router.get("/", frontpage);
	router.post("/createpoll", create_poll);
	router.get("/storage/:filename", file_handler);
	router.get("/vote/:value", see_poll);
	router.post("/votefor", vote_answer);
	// router.get("/votedone", vote_stats);
	router.get("/:value", vote_handler);
	router
}

