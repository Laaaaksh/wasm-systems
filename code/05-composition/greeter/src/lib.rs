#[allow(warnings)]
mod bindings;

use bindings::wasm_systems::namer::names::get_name;
use bindings::Guest;

struct Component;

impl Guest for Component {
    fn greet() -> String {
        let name = get_name();
        format!("Hello, {name}!")
    }
}

bindings::export!(Component with_types_in bindings);
