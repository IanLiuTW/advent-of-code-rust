# Usage:
# Make sure the session credential is setup as instructed in the README.
# Set up the day first with `export DAY=<DAY>` or just run `DAY=05 make <COMMAND>`.

create:
	cargo scaffold $(DAY) --download
test:
	cargo test --bin $(DAY) -- --nocapture
solve:
	cargo solve $(DAY) -- --nocapture
submit1:
	cargo solve $(DAY) --submit 1
submit2:
	cargo solve $(DAY) --submit 2
