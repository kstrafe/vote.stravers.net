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
				h1 { "Hello world!" }
				div class="row" {
					div class="small-2 columns" {
						"Hey!"
					}
					div class="small-2 columns" {
						"There"
					}
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
