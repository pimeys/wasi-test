use bindings::{ErrorResponse, Guest, Headers};

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    fn request_callback(headers: Headers) -> Result<(), ErrorResponse> {
        dbg!(headers.get("kekw"));
        headers.set("Foo", "bar");

        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
