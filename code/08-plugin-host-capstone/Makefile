CARGO_COMPONENT := cargo component
CARGO := cargo
WASM_TOOLS := wasm-tools

PLUGINS_DIR := plugins-built
HOST_OUT := host/target/release/host

.PHONY: all build build-plugins build-host run wit clean

all: run

build-plugins:
	cd plugins/shout && $(CARGO_COMPONENT) build --release
	cd plugins/reverse && $(CARGO_COMPONENT) build --release
	mkdir -p $(PLUGINS_DIR)
	cp plugins/shout/target/wasm32-wasip1/release/shout.wasm $(PLUGINS_DIR)/
	cp plugins/reverse/target/wasm32-wasip1/release/reverse.wasm $(PLUGINS_DIR)/

build-host:
	cd host && $(CARGO) build --release

build: build-plugins build-host

# Loads every .wasm file in plugins-built/, calling each one's `name` and
# `transform` exports with the same input string - the host has no
# compile-time knowledge of shout.wasm or reverse.wasm specifically.
run: build
	./$(HOST_OUT) $(PLUGINS_DIR) "Hello, plugins!"

# Confirms the claim in the README: even a plugin whose own WIT declares
# zero WASI ends up importing the full wasi:cli world once built.
wit: build-plugins
	$(WASM_TOOLS) component wit $(PLUGINS_DIR)/shout.wasm

clean:
	cd plugins/shout && cargo clean
	cd plugins/reverse && cargo clean
	cd host && cargo clean
	rm -rf $(PLUGINS_DIR)
