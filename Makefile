.PHONY: prepare prepare-backend prepare-frontend check check-quiet format format-check test audit backend frontend cargo-update

prepare:
	./configs/git/setup.sh
	make prepare-backend
	#make prepare-frontend
prepare-backend:
	cd modules/backend && cargo sqlx prepare
prepare-frontend:
	cd modules/backend && yarn prepare
backend:
	cd modules/backend && cargo run
frontend:
	cd modules/frontend && yarn dev
check:
	cd modules/backend && cargo clippy
check-ci:
	cd modules/backend && cargo clippy --all-features --all-targets --quiet
format:
	cd modules/backend && cargo fmt
format-check:
	cd modules/backend && ../../configs/scripts/cargo-fmt.sh
test:
	#cd modules/backend && cargo test && cargo test -- --ignored # add virtual postgres for stable API tests
	cd modules/backend && cargo test
test-ci:
	cd modules/backend && cargo test
audit:
	cd modules/backend && cargo audit
cargo-update:
	cd modules/backend && cargo update