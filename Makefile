all: install

build:
	cargo build --release
install: build
	-rm "$(SFPROC_BIN)/sfproc"
	cp -v target/release/sfproc "$(SFPROC_BIN)"
test:
	cargo test
docker-build:
	docker build -t sfproc .
docker-run:
	docker run -it --rm --name sfproc sfproc --help
