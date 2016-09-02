use iron::prelude::*;
use iron::status;
use router::Router;
use hbs::{Template, HandlebarsEngine, DirectorySource, MemorySource};
use std::collections::BTreeMap;
//use std::error::Error;

pub fn create(req: &mut Request) -> IronResult<Response> {
    info!("Session Post");
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let mut data = BTreeMap::new();
    data.insert("title".to_string(), "Wow that worked!".to_string());

    resp.set_mut(Template::with("<h1>{{title}}</h1>", data)).set_mut(status::Ok);
    Ok(resp)
}

pub fn index(_: &mut Request) -> IronResult<Response> {
    let mut data = BTreeMap::new();
    data.insert("title".to_string(), "Wow that worked!".to_string());
    let mut resp = Response::new();

    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}