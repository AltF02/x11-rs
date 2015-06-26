all: tests docs

docs: docs.x11 docs.x11-dl

docs.x11:
	cd x11 && cargo doc --verbose

docs.x11-dl:
	cd x11-dl && cargo doc --verbose

tests: test.x11 test.x11-dl

test.x11:
	cd x11 && cargo build --verbose && cargo test --verbose

test.x11-dl:
	cd x11-dl && cargo build --verbose && cargo test --verbose

regenerate:
	mkdir -p src/generated
	cd x11 && REGENERATE_BINDINGS=1 cargo build
