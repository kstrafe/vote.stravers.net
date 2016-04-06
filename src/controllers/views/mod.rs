pub fn render() -> String {
	let mut buffer = String::new();
	html! {
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
	};
	buffer
}
