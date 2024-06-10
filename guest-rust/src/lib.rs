#[allow(warnings)]
mod bindings;

use bindings::{Guest, Headers};

struct Component;

impl Guest for Component {
    fn request_callback(headers: Headers) -> Result<(), String> {
        headers.set("Foo", "bar");

        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
