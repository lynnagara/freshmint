serve:
	cd server && cargo run
.PHONY: serve

test-server:
	cd server && cargo test -- --nocapture
.PHONY: test-server

