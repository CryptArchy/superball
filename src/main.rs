#![feature(slice_patterns)]
#![cfg_attr(all(feature="serde_type"), feature(custom_derive, plugin))]
#![cfg_attr(all(feature="serde_type"), plugin(serde_macros))]

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
#[cfg(not(feature = "serde_type"))]
extern crate rustc_serialize;
#[cfg(feature = "serde_type")]
extern crate serde;
#[cfg(feature = "serde_type")]
extern crate serde_json;
#[macro_use]
extern crate maplit;

use iron::prelude::*;
use router::Router;
use hbs::{Template, HandlebarsEngine, DirectorySource, MemorySource};
use std::error::Error;

mod handlers;
mod storage;

fn main() {
    log4rs::init_file("log4rs.toml", Default::default()).unwrap();
    let ver: version::Version = std::str::FromStr::from_str(version!()).unwrap();
    println!("Superball v{}", ver);
    info!("Superball v{}", ver);

    let mut router = Router::new();
    router.get("/", handlers::home, "root");
    router.get("/:query", handlers::echo, "echo");
    router.get("/bounce", handlers::bounce, "bounce");
    router.get("/session", handlers::session::index, "session");
    router.get("/session/test", handlers::session::test, "session/test");
    router.post("/session", handlers::session::create, "session:post");

    // HandlebarsEngine will look up all files with "./pub/templates/**/*.hbs"
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./pub/templates/", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }

    let mut chain = Chain::new(router);

    chain.link_after(hbse);

    Iron::new(chain).http("0.0.0.0:8080").unwrap();
}
