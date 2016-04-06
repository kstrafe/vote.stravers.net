pub fn render() -> String {
	let mut buffer = String::new();
	html! {
		buffer,
		html {
			body {
				p "Oatmeal"
			}
		}
	};
	buffer
}
