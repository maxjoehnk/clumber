run: modules
	cd runner && cargo run

modules: module-docker

module-docker:
	cd runner/modules/docker && cargo build --release
