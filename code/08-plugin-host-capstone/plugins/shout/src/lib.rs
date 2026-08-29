#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn name() -> String {
        "shout".to_string()
    }

    fn transform(input: String) -> String {
        format!("{}!", input.to_uppercase())
    }
}

bindings::export!(Component with_types_in bindings);
