pub mod views;
pub mod index;

use iron::prelude::*;
use urlencoded::{QueryMap, UrlEncodedBody, UrlEncodedQuery};

#[allow(dead_code)]
pub fn get<'a>(req: &'a mut Request) -> Option<&'a QueryMap> {
	debug!("Fetching GET parameters");
	match req.get_ref::<UrlEncodedQuery>() {
		Ok(hashmap) => {
			debug!("Succesfully gotten GET parameters");
			Some(hashmap)
		}
		Err(err) => {
			debug!("Failed getting GET parameters: {:?}", err);
			None
		}
	}
}

#[allow(dead_code)]
pub fn post<'a>(req: &'a mut Request) -> Option<&'a QueryMap> {
	debug!("Fetching POST parameters");
	match req.get_ref::<UrlEncodedBody>() {
		Ok(hashmap) => {
			debug!("Succesfully gotten POST parameters");
			Some(hashmap)
		}
		Err(err) => {
			debug!("Failed getting POST parameters: {:?}", err);
			None
		}
	}
}
