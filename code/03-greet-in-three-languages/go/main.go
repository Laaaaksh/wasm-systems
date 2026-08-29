package main

import (
	"fmt"

	"go.bytecodealliance.org/cm"

	"greet-cli-go/gen/wasi/cli/environment"
	"greet-cli-go/gen/wasi/cli/run"
)

// Same `../wit/world.wit` as the Rust and Python implementations in this
// directory - `gen/` is generated from it by wit-bindgen-go (see the
// Makefile's `gen` target) and is not committed, the same way Rust's
// bindings.rs and Python's wit_world/ aren't.
func init() {
	run.Exports.Run = func() cm.BoolResult {
		args := environment.GetArguments().Slice()
		// args[0] is argv[0], same convention as the Rust and Python
		// implementations.
		name := "world"
		if len(args) > 1 {
			name = args[1]
		}
		fmt.Printf("Hello, %s, from Go!\n", name)
		return cm.BoolResult(cm.ResultOK)
	}
}

func main() {}
