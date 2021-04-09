
test: export RUST_BACKTRACE = 0
test:
	cargo test --features "color_from_css color_quantization experimental"
