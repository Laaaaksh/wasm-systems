CARGO_COMPONENT := cargo component
CARGO := cargo

GUEST_OUT := guest/target/wasm32-wasip1/release/guest.wasm
RUNNER_OUT := runner/target/release/runner

.PHONY: all build build-guest build-runner run clean

all: run

build-guest:
	cd guest && $(CARGO_COMPONENT) build --release

build-runner:
	cd runner && $(CARGO) build --release

build: build-guest build-runner

# The runner prints two runs of the *same* guest.wasm: once with no
# filesystem grant, once with ./granted-dir preopened read-only. Nothing
# about guest.wasm changes between the two - only what the host decided to
# hand it.
run: build
	./$(RUNNER_OUT) $(GUEST_OUT) granted-dir

clean:
	cd guest && cargo clean
	cd runner && cargo clean
