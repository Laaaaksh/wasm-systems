CARGO_COMPONENT := cargo component
WASMTIME := wasmtime
WASM_TOOLS := wasm-tools

OUT := target/wasm32-wasip1/release/hello-component-rust.wasm

.PHONY: all build run wit validate clean
all: build

build:
	$(CARGO_COMPONENT) build --release

run: build
	$(WASMTIME) run $(OUT) WASI

# Shows the world this component actually implements - note the wasi:cli
# imports cargo-component wired in automatically, with no WIT file of our
# own. Compare to 05-richer-wit-world, where we author the WIT by hand.
wit: build
	$(WASM_TOOLS) component wit $(OUT)

validate: build
	$(WASM_TOOLS) validate --features component-model $(OUT)
	@echo "OK: $(OUT) is a valid component"

clean:
	cargo clean
