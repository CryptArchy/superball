#![feature(slice_patterns)]

#[macro_use]
extern crate version;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate iron;
extern crate handlebars_iron as hbs;
extern crate rusqlite;
extern crate router;
extern crate time;
extern crate urlencoded;

use iron::prelude::*;
use router::Router;
use hbs::{Template, HandlebarsEngine, DirectorySource, MemorySource};
use std::error::Error;

mod handlers;
mod storage;

//#![cfg_attr(all(feature="serde_type"), feature(custom_derive, plugin))]
//#![cfg_attr(all(feature="serde_type"), plugin(serde_macros))]

//#[cfg(not(feature = "serde_type"))]
//extern crate rustc_serialize;
//#[cfg(feature = "serde_type")]
//extern crate serde;
//#[cfg(feature = "serde_type")]
//extern crate serde_json;
//#[macro_use]
//extern crate maplit;

fn main() {
    log4rs::init_file("log4rs.toml", Default::default()).unwrap();
    let ver: version::Version = std::str::FromStr::from_str(version!()).unwrap();
    println!("Superball v{}", ver);

    let mut router = Router::new();
    router.get("/", handlers::home, "root");
    router.get("/:query", handlers::echo, "echo");
    router.get("/bounce", handlers::bounce, "bounce");
    //router.post("/session", handlers::session::create);
    router.get("/session", handlers::session::index, "session");
    router.get("/session/test", handlers::session::get, "session/test");

    // HandlebarsEngine will look up all files with "./pub/templates/**/*.hbs"
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./pub/templates/", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }

    let mut chain = Chain::new(router);

    chain.link_after(hbse);

    Iron::new(chain).http("localhost:3030").unwrap();
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
        let qparts: Vec<&str> = raw_query.split('&').collect();

        assert_eq!(qparts[0], "id=1");
        assert_eq!(qparts[1], "name=foo");

        let mut query = HashMap::new();
        for qp in qparts {
            let kv: Vec<&str> = qp.split('=').collect();
            query.insert(kv[0], kv[1]);
        }

        assert_eq!(query["id"], "1");
        assert_eq!(query["name"], "foo");
    }
}
