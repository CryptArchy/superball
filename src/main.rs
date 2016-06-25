#[macro_use]
extern crate version;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    log4rs::init_file("log4rs.toml", Default::default()).unwrap();
    let ver: version::Version = std::str::FromStr::from_str(version!()).unwrap();
    println!("Superball v{}", ver);

    let mut router = Router::new();
    router.get("/", home_handler);
    router.get("/:query", echo_handler);
    router.get("/bounce", bounce_handler);

    fn home_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Welcome to Superball Bounce Server!")))
    }

    fn bounce_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Bounce!")))
    }

    fn echo_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }

    Iron::new(router).http("localhost:3030").unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_test() {
        assert!(true);
        assert_eq!(1,1);
    }
}
