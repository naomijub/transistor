crux:
	docker run -d -p 3000:3000 --name CruxDB juxt/crux-standalone:20.02-1.7.0-alpha

mock:
	cargo test --test lib --no-fail-fast --features "mock"