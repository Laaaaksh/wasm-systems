CARGO_COMPONENT := cargo component
WASMTIME := wasmtime
WASM_TOOLS := wasm-tools

OUT := target/wasm32-wasip1/release/richer_wit_world.wasm

.PHONY: all build demo wit clean
all: build

build:
	$(CARGO_COMPONENT) build --release

# Every one of these exercises the Canonical ABI lifting/lowering a
# different value shape: a bare float, a tuple, a list of records, an
# error variant, and a plain string result.
demo: build
	@echo "circle, radius 2:"
	@$(WASMTIME) run --invoke 'area(circle(2.0))' $(OUT)
	@echo
	@echo "rectangle, 3x4:"
	@$(WASMTIME) run --invoke 'area(rectangle((3.0, 4.0)))' $(OUT)
	@echo
	@echo "circle, negative radius (error path):"
	@$(WASMTIME) run --invoke 'area(circle(-1.0))' $(OUT)
	@echo
	@echo "polygon, a 4x3 rectangle traced as 4 points:"
	@$(WASMTIME) run --invoke 'area(polygon([{x: 0.0, y: 0.0}, {x: 4.0, y: 0.0}, {x: 4.0, y: 3.0}, {x: 0.0, y: 3.0}]))' $(OUT)
	@echo
	@echo "polygon, too few points (error path):"
	@$(WASMTIME) run --invoke 'area(polygon([{x: 0.0, y: 0.0}, {x: 1.0, y: 1.0}]))' $(OUT)
	@echo
	@echo "describe:"
	@$(WASMTIME) run --invoke 'describe(circle(2.0))' $(OUT)

wit: build
	$(WASM_TOOLS) component wit $(OUT)

clean:
	cargo clean
