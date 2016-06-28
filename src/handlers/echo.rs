use iron::prelude::*;
use iron::status;
use router::Router;

pub fn echo_handler(req: &mut Request) -> IronResult<Response> {
    info!("Echo");
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}
