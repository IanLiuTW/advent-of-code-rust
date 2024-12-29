create:
	cargo scaffold $(DAY) --download
test:
	cargo test --bin $(DAY) -- --nocapture
solve:
	cargo solve $(DAY) -- --nocapture
