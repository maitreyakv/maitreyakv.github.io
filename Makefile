export RUSTFLAGS=--cfg getrandom_backend="wasm_js"

check: 
	cargo check && cargo clippy

serve:
	trunk serve --clear -a 0.0.0.0

release:
	trunk build --release

clean:
	trunk clean
	cargo clean
