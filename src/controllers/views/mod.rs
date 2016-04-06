pub fn render() -> String {
	let mut buffer = String::new();
	match html! {
		buffer,
		html {
			head {
				link rel="stylesheet" type="text/css" href="file/reset.css" /
				link rel="stylesheet" type="text/css" href="file/style.css" /
			}
			body {
				div class="fullscreen yellow" {}
				div class="fullscreen green" {}
			}
		}
	} {
		Ok(()) => trace!("Generated Html"),
		Err(err) => error!("Unable to parse Html: {:?}", err),
	}
	buffer
}
