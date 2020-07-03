X11_FEATURES=xlib

all: build-x11 build-x11-dl

build: build-x11 build-x11-dl

build-x11:
	cd x11 && cargo build --features "${X11_FEATURES}"

build-x11-dl:
	cd x11-dl && cargo build

tests: tests-x11 tests-x11-dl tests-x11-nostd

tests-x11:
	cd x11 && cargo test

tests-x11-dl:
	cd x11-dl && cargo test

tests-x11-nostd:
	cd x11 && cargo test --no-default-features

docs: docs-x11 docs-x11-dl

docs-x11:
	cd x11 && cargo doc --features "${X11_FEATURES}"

docs-x11-dl:
	cd x11-dl && cargo doc
