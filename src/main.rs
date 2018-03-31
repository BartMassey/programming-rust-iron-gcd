// Copyright (c) 2018 Bart Massey

// Iron-gcd example from Blandy & Orendorff, ch 1.
// Webserver provides a GCD function.

extern crate iron;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;

// Start a webserver offering a form.
fn main() {
    println!("Serving on http://localhost:3000...");
    let _ = Iron::new(get_form).http("localhost:3000")
        .expect("could not start iron server");
}

// Provide form for webserver.
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method = "post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
        </form>
    "#);

    Ok(response)
}


