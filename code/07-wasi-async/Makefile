CARGO := cargo
WASM_TOOLS := wasm-tools
WASMTIME := wasmtime

OUT := target/wasm32-wasip2/release/wasi_async_demo.wasm

.PHONY: all build wit validate try-run clean

all: build

build:
	rustup target add wasm32-wasip2 >/dev/null 2>&1 || true
	$(CARGO) build --release --target wasm32-wasip2

# Confirms this is a real, valid component with a genuine stream<u32>
# export - the part of this sample that *does* work today.
validate: build
	$(WASM_TOOLS) validate --features component-model $(OUT)
	@echo "OK: $(OUT) is a valid component exporting count-to: func(u32) -> stream<u32>"

wit: build
	$(WASM_TOOLS) component wit $(OUT)

# This target is expected to fail - see README.md. It's here so you can
# reproduce the exact current failure yourself instead of taking this
# repo's word for it.
try-run: build
	$(WASMTIME) run -W component-model-async=y --invoke 'count-to(3)' $(OUT)

clean:
	cargo clean
