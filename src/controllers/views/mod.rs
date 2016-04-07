pub fn render() -> String {
	let mut buffer = String::new();
	match html! {
		buffer,
		html {
			head {
				script type="text/javascript" src="file/jquery-2.2.3.min.js" {}
				link rel="stylesheet" type="text/css" href="file/reset.css" /
				link rel="stylesheet" type="text/css" href="file/style.css" /
			}
			body {
				div class="fullscreen bg-navy" {
					div class="center" {
						h1 class="welcome" id="welcome" {
							"Welcome!"
						}
					}
				}
				div class="fullscreen bg-green" {}
				script type="text/javascript" src="file/script.js" {}
			}
		}
	} {
		Ok(()) => trace!("Generated Html"),
		Err(err) => error!("Unable to parse Html: {:?}", err),
	}
	buffer
}
