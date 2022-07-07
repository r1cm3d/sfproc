all: install

build:
	cargo build --release
install: build
	-rm "$(SFPROC_BIN)/sfproc"
	cp -v target/release/sfproc "$(SFPROC_BIN)"
test:
	cargo test

