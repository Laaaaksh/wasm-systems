#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn name() -> String {
        "reverse".to_string()
    }

    fn transform(input: String) -> String {
        input.chars().rev().collect()
    }
}

bindings::export!(Component with_types_in bindings);
