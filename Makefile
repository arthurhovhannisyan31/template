.PHONY: prepare prepare-backend prepare-frontend check check-ci check-quiet format format-check test test-ci audit backend frontend cargo-update

prepare:
	./configs/git/setup.sh
	make prepare-backend
	#make prepare-frontend
prepare-backend:
	cd modules/backend && cargo sqlx prepare
prepare-frontend:
	cd modules/frontend && yarn prepare
backend:
	cd modules/backend && cargo run
frontend:
	cd modules/frontend && yarn dev
check:
	cd modules/backend && cargo clippy
	cd modules/frontend && yarn check
check-ci:
	cd modules/backend && cargo clippy --all-features --all-targets --quiet
	cd modules/frontend && yarn check
format:
	cd modules/backend && cargo fmt
	cd modules/frontend && yarn format
format-check:
	cd modules/backend && ../../configs/scripts/cargo-fmt.sh
test:
	#cd modules/backend && cargo test && cargo test -- --ignored # add virtual postgres for stable API tests
	cd modules/backend && cargo test
test-ci:
	cd modules/backend && cargo test
audit:
	cd modules/backend && cargo audit
	cd modules/frontend && yarn npm audit
cargo-update:
	cd modules/backend && cargo update