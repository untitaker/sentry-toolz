install:
	cargo build --release
	ln -sf $$PWD/target/release/sentry-toolz ~/.local/bin/
	ln -sf $$PWD/scripts/* ~/.local/bin/
