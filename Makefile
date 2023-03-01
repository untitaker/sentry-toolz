install:
	cargo build --release
	ln -s target/release/sentry-toolz ~/.local/bin/
	ln -s scripts/* ~/.local/bin/
