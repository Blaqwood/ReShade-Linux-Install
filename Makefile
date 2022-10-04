DESTDIR = /usr/local

cmd:
	mkdir build
	cargo build --release --bin cmd
	cp -r target/release/cmd build/
gui:
	mkdir build
	cargo build --release --bin gui
	cp -r target/release/gui build/