crux:
	docker run -d -p 3000:3000 --name CruxDB juxt/crux-standalone:20.07-1.10.0

int:
	cargo test --test lib --no-fail-fast --features "mock time"

unit:
	cargo test --locked  --no-fail-fast --lib

test: unit int