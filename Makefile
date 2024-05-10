run:
	@echo "Running server..."
	@RUST_LOG=debug cargo run

build:
	@echo "Building server..."
	@cd logo-mosaic-web && pnpm install && pnpm run build
	@cargo build --release