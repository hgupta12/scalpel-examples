.PHONY: build run

build:
	wasm-pack build --target web --features wasm

	rm -rf static/pkg/
	mv pkg/ static/
	rm -rf pkg/

	cargo build --release

	@echo "Build completed successfully!"

run:
	cargo run