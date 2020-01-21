// Copyright © 2018 Jim Blandy, Jason Orendorff, Bart Massey
// This work is released under the "MIT License".
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Iron-gcd example from Blandy & Orendorff, ch 1.
// Webserver provides a GCD function.

use iron::prelude::*;
use iron::status;
use mime::*;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

/// Generate a response with a given status and message.
fn response(status: status::Status, msg: String) -> Response {
    let mut r = Response::new();
    r.set_mut(status);
    r.set_mut(msg);
    r
}

/// Generate a response to a failed request.
fn response_bad(msg: String) -> IronResult<Response> {
    Ok(response(status::BadRequest, msg))
}

/// Generate a response to successful request.
fn response_ok(msg: String) -> IronResult<Response> {
    let mut r = response(status::Ok, msg);
    r.set_mut(mime!(Text/Html; Charset=Utf8));
    Ok(r)
}

/// Handle a GCD POST request.
fn post_gcd(request: &mut Request) -> IronResult<Response> {
    // Get the form data from the client.
    let data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            return response_bad(format!(
                "Error parsing form data: {:?}\n",
                e
            ));
        }
        Ok(data) => data,
    };

    // Get the "n" field from the form data.
    let data = match data.get("n") {
        None => {
            return response_bad(
                "No numbers (\"n\") in form data\n".to_string(),
            );
        }
        Some(data) => data,
    };

    // Parse the numbers and apply GCD.
    let mut result = None;
    for d in data {
        match u64::from_str(&d) {
            Err(_) => {
                return response_bad(format!("Bad number n={}\n", d));
            }
            Ok(d) => {
                result = match result {
                    None => Some(d),
                    Some(r) => Some(gcd(r, d)),
                }
            }
        }
    }

    // On failure, return a failure response.
    if result.is_none() {
        return response_bad("No numbers given\n".to_string());
    }

    // Return a renderable HTML result.
    response_ok(format!("GCD is {}", result.unwrap()))
}

/// Provide form for webserver. XXX Hardcoding HTML isn't
/// usually the right plan: this is just a demo.
fn get_form(_request: &mut Request) -> IronResult<Response> {
    response_ok(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method = "post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
        </form>
        "#
        .to_string(),
    )
}

/// Compute the GCD of two numbers.
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t
        }
        m %= n
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    let n1 = 2 * 3 * 5 * 11 * 17;
    let n2 = 3 * 7 * 11 * 13 * 19;
    let d = 3 * 11;
    assert_eq!(gcd(n1, n2), d)
}

/// Start a webserver offering a form.
fn main() {
    // The router determines what function will be called
    // based on the path in the URL.
    let mut router = Router::new();

    // Route to put up the form.
    router.get("/", get_form, "root");
    // Route to handle form data.
    router.post("/gcd", post_gcd, "gcd");

    // Start the server.
    println!("Serving on http://localhost:3000...");
    let _ = Iron::new(router)
        .http("localhost:3000")
        .expect("could not start iron server");
}
