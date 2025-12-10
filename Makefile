# Usage:
# Make sure the session credential is setup as instructed in the README.
# Set up the day first with `export d=<DAY>` or just run `d=05 make <COMMAND>`.

create:
	cargo scaffold $(d) --download
test:
	cargo test --bin $(d) -- --nocapture
solve:
	cargo solve $(d) -- --nocapture
submit1:
	cargo solve $(d) --submit 1
submit2:
	cargo solve $(d) --submit 2
