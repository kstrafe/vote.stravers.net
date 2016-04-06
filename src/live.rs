/*
use std;
use std::convert::Into;
use std::io::BufRead;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

#[derive(Clone, Debug)]
pub enum CommandType {
	Empty,
	Get(String),
	Set(String, String),
}

fn classify_string(command: &str) -> CommandType {
	let parts = &mut command.splitn(2, '=');
	match parts.clone().count() {
		0 => CommandType::Empty,
		1 => CommandType::Get(command.trim().into()),
		2 => {
			let error = "This can never happen because we check the amount of parts in the iterator. Defaulting to an Empty sequence";
			let left = match parts.next() {
				Some(left) => left.trim(),
				None => {
					error!("Left, {}", error);
					return CommandType::Empty;
				}
			};
			let right = match parts.next() {
				Some(right) => right.trim(),
				None => {
					error!("Right, {}", error);
					return CommandType::Empty;
				}
			};
			CommandType::Set(left.into(), right.into())
		}
		_ => {
			error!("Count not 0, 1, or 2. Indicating an splitn method error. Defaulting to no operation");
			CommandType::Empty
		}
	}
}

macro_rules! register_live {
	($left:expr, $right:expr, $name:ident) => {{
		if *$left == stringify!($name) {
			$name = $right.clone();
		}
		debug!("The string is {}", $name);
	}};
	($e:expr => $name:ident) => {{
		if $e == stringify!($name) {
			info!("{}: {:?}", stringify!($name), $name);
		}
	}};
	($e:expr => $($name:ident),*) => {{
		$(
			register_live!($expr => $name);
		)*
	}};
	($left:expr, $right:expr => $( $name:ident ),*) => {{
		$(
			register_live!(&$left, &$right, $name);
		)*
	}};
}


pub fn create_channels()
		-> (Receiver<CommandType>, JoinHandle<()>)  {
	use std::thread;
	use std::sync::mpsc::channel;
	use std::io::BufRead;
	use std::io;

	let (send, recv) = channel();
	let stdin = io::stdin();
	let thread = thread::spawn(move || {
		let mut stdin = stdin.lock();
		'looper: loop {
			let command = {
				let mut command = String::new();
				match stdin.read_line(&mut command) {
					Ok(bytes) => debug!("Read {} stdin bytes to string", bytes),
					Err(err) => {
						error!("Unable to read std: {:?},\nExiting interpreter", err);
						break 'looper;
					}
				}
				command
			};

			{
				let arg = classify_string(&command);
				match send.send(arg) {
					Ok(()) => debug!("Message sent"),
					Err(err) => error!("Unable to send to channel: {:?}", err),
				}
			}
		}
	});
	(recv, thread)
}

pub fn read_handler() {

	let (recv, thread) = create_channels();

	let mut fps = String::from("30");

	loop {
		use std::sync::mpsc::TryRecvError;
		match recv.try_recv() {
			Ok(message) => {
				trace!("Gotten message {:?}", message);
				match message {
					CommandType::Empty => {}
					CommandType::Get(string) => {
						register_live!(string => fps);
					}
					CommandType::Set(left, right) => {
						register_live!(left, right
							=> fps, fps);
					}
				}
			}
			Err(TryRecvError::Empty) => {}
			Err(TryRecvError::Disconnected) => error!("Sender dc'd"),
		}
		std::thread::sleep(std::time::Duration::new(0, 500));
	}
	match thread.join() {
		Ok(()) => debug!("Joined the stdin thread"),
		Err(err) => error!("Unable to join the stdin thread {:?}", err),
	}
}
*/
