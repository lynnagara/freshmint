serve:
	cd server && cargo run
.PHONY: serve

test-server:
	cd server && cargo test -- --nocapture
.PHONY: test-server

build-ui:
	cd client && yarn run build
.PHONY: build-ui

watch-ui:
	cd client && yarn run watch
.PHONY: watch-ui

