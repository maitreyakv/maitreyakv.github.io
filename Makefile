serve:
	trunk serve --clear

release:
	trunk build --release

clean:
	trunk clean
	cargo clean
