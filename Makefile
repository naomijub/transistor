crux:
	docker run -d -p 3000:3000 --name CruxDB juxt/crux-standalone:20.09-1.11.0

int:
	cargo test --test lib --no-fail-fast --features "mock"

unit:
	cargo test --locked  --no-fail-fast --lib

examples-sync:
	cargo test --examples

examples-async:
	cargo test --examples --features "async"

test: unit int examples-sync examples-async