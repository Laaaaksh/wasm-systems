#[allow(warnings)]
mod bindings;

use bindings::exports::wasm_systems::namer::names::Guest;

struct Component;

impl Guest for Component {
    fn get_name() -> String {
        "Ferris".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
