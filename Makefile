.PHONY: default build clean clippy release test update

CARGO_FLAGS := --features "$(NS_FEATURES)"
SPECIAL_FILES := examples/asr_simple.rs

default: build

build:
	cargo build $(CARGO_FLAGS)

clean:
	cargo clean
	cargo clean --release

clippy:
	if $$CLIPPY; then cargo clippy $(CARGO_FLAGS); fi

release:
	cargo build --release $(CARGO_FLAGS)

test: build
	cargo test $(CARGO_FLAGS)

update:
	cargo update
	RENEW_SDK="1" cargo build $(CARGO_FLAGS)
