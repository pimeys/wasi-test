run:
	cd guest-rust && cargo component build --release
	cd host-rust && cargo run
