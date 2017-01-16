extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::typemap::Key;

pub fn echo(req: &mut Request) -> IronResult<Response> {
    match req.url.path().len() == 2 {
        true => Ok(Response::with((status::Ok, req.url.path()[1]))),
        _ => Ok(Response::with((status::Ok, "ECHO! World!"))),
    }
}
