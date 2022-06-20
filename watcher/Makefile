.PHONY: all test
	
all:
	cargo build

run: all
	PWD=`pwd` ./target/debug/`basename ${PWD}`

format:
	find src tests -type f -exec rustfmt {} \;

test:
	cargo test -- --nocapture
