RUST_OUT := rust/target/wasm32-wasip1/release/greet-cli-rust.wasm
PY_OUT := python/greet_cli_py.wasm
GO_OUT := go/greet_cli_go.wasm

WASMTIME := wasmtime

.PHONY: all build run clean build-rust build-python build-go run-rust run-python run-go validate

all: build

build: build-rust build-python build-go

build-rust:
	cd rust && cargo component build --release

# componentize-py needs the WIT dir passed explicitly with -d, and the
# world with -w - both must match ../wit/world.wit's package/world names.
build-python:
	cd python && componentize-py -d ../wit -w greet-cli componentize app -o greet_cli_py.wasm

# wit-bindgen-go generates Go source from the WIT world; TinyGo then
# compiles that (plus main.go) straight to a component with --wit-package/
# --wit-world telling it which world's imports/exports to wire up.
build-go:
	cd go && wit-bindgen-go generate -w greet-cli -o gen --package-root greet-cli-go ../wit
	cd go && tinygo build -target=wasip2 --wit-package ../wit --wit-world greet-cli -o greet_cli_go.wasm .

# Three unrelated toolchains, one WIT world, one runtime, the same shape
# of output - that's the whole point of this sample.
run: build
	@echo "== rust/ (cargo component) =="
	@$(WASMTIME) run $(RUST_OUT) Ferris
	@echo
	@echo "== python/ (componentize-py) =="
	@$(WASMTIME) run $(PY_OUT) Guido
	@echo
	@echo "== go/ (TinyGo + wit-bindgen-go) =="
	@$(WASMTIME) run $(GO_OUT) Gopher

run-rust: build-rust
	$(WASMTIME) run $(RUST_OUT) Ferris

run-python: build-python
	$(WASMTIME) run $(PY_OUT) Guido

run-go: build-go
	$(WASMTIME) run $(GO_OUT) Gopher

validate: build
	wasm-tools validate --features component-model $(RUST_OUT)
	wasm-tools validate --features component-model $(PY_OUT)
	wasm-tools validate --features component-model $(GO_OUT)
	@echo "OK: all three are valid components"

clean:
	cd rust && cargo clean
	rm -rf python/wit_world python/componentize_py_async_support \
	       python/componentize_py_types.py python/componentize_py_runtime.pyi \
	       python/poll_loop.py python/greet_cli_py.wasm
	rm -rf go/gen $(GO_OUT)
