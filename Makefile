.PHONY: debug build-release test

ARGS=p1
# ARGS=-l -d <some_path>

run:
	cargo run  -q --bin ${ARGS}

debug:
	RUST_BACKTRACE=true cargo run -q --bin ${ARGS}

build-release:
	cargo build --release

test:
	cargo test --workspace

fmt:
	cargo fmt -- --check

check: fmt test

install:
	cargo install --path "." --offline

install-timing:
	cargo install --features=timing --path "." --offline

