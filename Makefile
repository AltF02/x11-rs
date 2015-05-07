all: tests docs

docs.x11:
	cd x11 && cargo doc --verbose

docs.x11-dl:
	cd x11-dl && cargo doc --verbose

test.x11:
	cd x11 && cargo build --verbose && cargo test --verbose

test.x11-dl:
	cd x11-dl && cargo build --verbose && cargo test --verbose