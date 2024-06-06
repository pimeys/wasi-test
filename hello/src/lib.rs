#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world(name: String) -> String {
        println!("stdout test");
        format!("Hello, {name}")
    }
}

bindings::export!(Component with_types_in bindings);
