all:
	cargo build
	RUST_LOG=ironweb=trace ./target/debug/ironweb
