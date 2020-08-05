# crux:
# 	docker run -d -p 3000:3000 --name CruxDB juxt/crux-standalone:20.02-1.7.0-alpha
crux:
	docker run -d -p 3000:3000 --name CruxDB juxt/crux-standalone:20.07-1.10.0

int:
	cargo test --test lib --no-fail-fast --features "mock"

unit:
	cargo test --locked  --no-fail-fast --lib

test: unit int