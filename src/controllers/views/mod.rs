use maud::PreEscaped;

pub fn render() -> String {
	let mut buffer = String::new();
	match html! {
		buffer,
		html {
			head {
				meta charset="utf-8" /
				link rel="stylesheet" type="text/css" href="file/reset.css" /
				link rel="stylesheet" type="text/css" href="file/css/foundation.css" /
				script type="text/javascript" src="file/jquery-2.2.3.min.js" {}
				title {
					"Rust on Iron"
				}
			}
			body {
				div class="top-bar" {
					div class="top-bar-left" {
						ul class="menu" {
							li class="menu-text" { "Hello!" }
							li { "Oy!" }
							li { a href="#" { "Test" }}
							li { a href="#" { "Test2" }}
						}
					}
					div class="top-bar-right" {
						ul class="menu" {
							li { input type="search" placeholder="Search" {}}
							li { button type="button" class="button" { "Search" }}
						}
					}
				}
				h1 { "Hello world!" }
				div class="row" {
					div class="small-2 large-4 columns" {
						"Hey!"
					}
					div class="small-4 large-4 columns" {
						"There"
					}
					div class="small-6 large-4 columns" {
						"There"
					}
				}
				div class="row" {
					button type="button" class="success button" { "Save" }
					button type="button" class="alert button" { "Delete" }
				}
				ul class="menu" {
					li { a href="#" { "One" }}
					li { a href="#" { "On4" }}
					li { a href="#" { "On5" }}
					li { a href="#" { "On6" }}
					li { a href="#" { "On7" }}
					li { a href="#" { "On8" }}
					li { a href="#" { "Tow" }}
				}
				script src="file/js/vendor/jquery.min.js" {}
				script src="file/js/vendor/what-input.min.js" {}
				script src="file/js/foundation.min.js" {}
				script {
					"$(document).foundation();"
				}
			}
		}
	} {
		Ok(()) => trace!("Generated Html"),
		Err(err) => error!("Unable to parse Html: {:?}", err),
	}
	buffer
}

pub fn render_not_found() -> String {
	let mut buffer = String::new();
	match html! {
		buffer,
		html {
			head {
				title {
					"Error"
				}
				style {
					^PreEscaped(".middle { margin-top: 50%; translate: transform(0, -50%); }")
					^PreEscaped("html { height: 100vh; text-align: center; vertical-align: middle; width: 100vw; }")
				}
			}
			body {
				h1 class="middle" {
					"Currently under maintenance, check back later :("
				}
			}
		}
	} {
		Ok(()) => trace!("Generated Html"),
		Err(err) => error!("Unable to parse Html: {:?}", err),
	}
	buffer
}
