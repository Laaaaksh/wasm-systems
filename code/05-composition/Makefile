CARGO_COMPONENT := cargo component
WAC := wac
WASMTIME := wasmtime
WASM_TOOLS := wasm-tools

NAMER := namer/target/wasm32-wasip1/release/namer.wasm
GREETER := greeter/target/wasm32-wasip1/release/greeter.wasm
COMPOSED := composed.wasm

.PHONY: all build compose run wit-before wit-after run-greeter-alone clean

all: run

build:
	cd namer && $(CARGO_COMPONENT) build --release
	cd greeter && $(CARGO_COMPONENT) build --release

compose: build
	$(WAC) plug $(GREETER) --plug $(NAMER) -o $(COMPOSED)

run: compose
	$(WASMTIME) run --invoke 'greet()' $(COMPOSED)

# Shows greeter.wasm's unsatisfied import before composition.
wit-before: build
	$(WASM_TOOLS) component wit $(GREETER)

# Shows the same world after wac has plugged namer's export into it - the
# wasm-systems:namer/names import is gone.
wit-after: compose
	$(WASM_TOOLS) component wit $(COMPOSED)

# Run greeter.wasm directly, with nothing providing its import - this is
# meant to fail; it's the "why compose at all" demonstration.
run-greeter-alone: build
	$(WASMTIME) run --invoke 'greet()' $(GREETER)

clean:
	cd namer && cargo clean
	cd greeter && cargo clean
	rm -f $(COMPOSED)
