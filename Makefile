.PHONY: prepare check check-quiet format format-check test audit backend

prepare:
	./configs/git/setup.sh
check:
	cd modules/backend && cargo clippy
check-ci:
	cd modules/backend && cargo clippy --all-features --all-targets --quiet
format:
	cd modules/backend && cargo fmt
format-check:
	cd modules/backend && ../../configs/scripts/cargo-fmt.sh
test:
	cd modules/backend && cargo test && cargo test -- --ignored
test-ci:
	cd modules/backend && cargo test
audit:
	cd modules/backend && cargo audit
backend:
	cd modules/backend && cargo run
cargo-update:
	cd modules/backend && cargo update