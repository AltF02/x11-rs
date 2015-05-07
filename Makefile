all: test.x11 test.x11-dl

test.x11:
	cd x11 && cargo build --verbose && cargo test --verbose

test.x11-dl:
	cd x11-dl && cargo build --verbose && cargo test --verbose