CARGO := cargo
WASMTIME := wasmtime
WASM_TOOLS := wasm-tools

NO_WASI_OUT := target/wasm32-unknown-unknown/release/hello.wasm
WASI_OUT := target/wasm32-wasip1/release/hello.wasm

.PHONY: all build run imports clean
all: build

build:
	rustup target add wasm32-unknown-unknown wasm32-wasip1 >/dev/null 2>&1 || true
	$(CARGO) build --release --target wasm32-unknown-unknown
	$(CARGO) build --release --target wasm32-wasip1

# Run both binaries so the difference is a printed fact, not a claim.
run: build
	@echo "== core wasm, no WASI ($(NO_WASI_OUT)) =="
	@echo "-- imports (none) --"
	@$(WASM_TOOLS) print $(NO_WASI_OUT) | grep -E '\(import' || echo "  (no imports)"
	@echo "-- output --"
	@$(WASMTIME) run $(NO_WASI_OUT) || true
	@echo "  (nothing was printed above: see this sample's README for why)"
	@echo
	@echo "== same source, wasm32-wasip1 ($(WASI_OUT)) =="
	@echo "-- imports (WASI syscalls) --"
	@$(WASM_TOOLS) print $(WASI_OUT) | grep -E '\(import'
	@echo "-- output --"
	@$(WASMTIME) run $(WASI_OUT)

clean:
	$(CARGO) clean
