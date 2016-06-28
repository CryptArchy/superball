#![feature(slice_patterns)]

#[macro_use]
extern crate version;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use router::Router;

mod handlers;

fn main() {
    log4rs::init_file("log4rs.toml", Default::default()).unwrap();
    let ver: version::Version = std::str::FromStr::from_str(version!()).unwrap();
    println!("Superball v{}", ver);

    let mut router = Router::new();
    router.get("/", handlers::home);
    router.get("/:query", handlers::echo);
    router.get("/bounce", handlers::bounce);

    Iron::new(router).http("localhost:3030").unwrap();
}

#[cfg(test)]
mod tests {
    use iron::Url;
    use std::collections::HashMap;

    #[test]
    fn parse_url() {
        // Don't do it this way, the urlencoded crate basically does this for you.
        let init_url = Url::parse("http://wwww.google.com?id=1&name=foo").unwrap();
        let ref raw_query = init_url.query.unwrap();
        let qparts : Vec<&str> = raw_query.split('&').collect();

        assert_eq!(qparts[0], "id=1");
        assert_eq!(qparts[1], "name=foo");

        let mut query = HashMap::new();
        for qp in qparts {
            let kv : Vec<&str> = qp.split('=').collect();
            query.insert(kv[0],kv[1]);
        }

        assert_eq!(query["id"], "1");
        assert_eq!(query["name"], "foo");
    }
}
