use iron::prelude::*;
use iron::status;

pub fn home_handler(_: &mut Request) -> IronResult<Response> {
    info!("Home");
    Ok(Response::with((status::Ok, "Welcome to Superball Bounce Server!")))
}
