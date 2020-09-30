crux:
	docker run -d -p 3000:3000 --name CruxDB juxt/crux-standalone:20.08-1.10.1

int:
	cargo test --test lib --no-fail-fast --features "mock"

unit:
	cargo test --locked  --no-fail-fast --lib

examples:
	cargo test --examples

test: unit int examples