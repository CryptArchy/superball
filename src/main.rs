#[macro_use]
extern crate version;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate iron;

use iron::prelude::*;
use iron::status;

mod router;

fn main() {
    log4rs::init_file("log4rs.toml", Default::default()).unwrap();
    let ver: version::Version = std::str::FromStr::from_str(version!()).unwrap();
    println!("Superball v{}", ver);

    let mut rtr = router::Router::new();

    rtr.add_route("".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Welcome to Superball Bounce Server!")))
    });

    rtr.add_route("bounce".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Bounce!")))
    });

    Iron::new(rtr).http("localhost:3030").unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_test() {
        assert!(true);
        assert_eq!(1,1);
    }
}
