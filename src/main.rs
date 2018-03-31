// Copyright (c) 2018 Bart Massey

// Iron-gcd example from Blandy & Orendorff, ch 1.
// Webserver provides a GCD function.

extern crate iron;
#[macro_use] extern crate mime;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        },
        Ok(data) => data,
    };

    let data = match data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("No numbers (\"n\") in form data\n"));
            return Ok(response);
        },
        Some(data) => data,
    };

    let mut result = None;
    for d in data {
        match u64::from_str(&d) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!("Bad number n={}\n", d));
                return Ok(response);
            },
            Ok(d) => {
                result = match result {
                    None => Some(d),
                    Some(r) => Some(gcd(r, d)),
                }
            }
        }
    }

    if let None = result {
        response.set_mut(status::BadRequest);
        response.set_mut(format!("No numbers given\n"));
        return Ok(response);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!("GCD is {}", result.unwrap()));
    Ok(response)
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

// Compute the GCD of two numbers.
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


// Start a webserver offering a form.
fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://localhost:3000...");
    let _ = Iron::new(router).http("localhost:3000")
        .expect("could not start iron server");
}
