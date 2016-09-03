use iron::prelude::*;
use iron::status;
use router::Router;
use hbs::{Template, HandlebarsEngine, DirectorySource, MemorySource};
use std::collections::BTreeMap;

use rustc_serialize::json::{ToJson, Json};

struct Link {
    title: String,
    href: String,
}

impl ToJson for Link {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("title".to_string(), self.title.to_json());
        m.insert("href".to_string(), self.href.to_json());
        m.to_json()
    }
}

fn make_data() -> BTreeMap<String, Json> {
    let mut data = BTreeMap::new();

    data.insert("title".to_string(), "Super Bounce!".to_json());

    let links =
        vec![Link {
                 title: "Complete".to_string(),
                 href: "http://localhost:55556/router/ClientCallBack.aspx?RIS=10".to_string(),
             },
             Link {
                 title: "Term".to_string(),
                 href: "http://localhost:55556/router/ClientCallBack.aspx?RIS=20".to_string(),
             },
             Link {
                 title: "Quality".to_string(),
                 href: "http://localhost:55556/router/ClientCallBack.aspx?RIS=30".to_string(),
             },
             Link {
                 title: "Overquota".to_string(),
                 href: "http://localhost:55556/router/ClientCallBack.aspx?RIS=40".to_string(),
             }];

    data.insert("links".to_string(), links.to_json());
    data.insert("engine".to_string(), "rustc_serialize".to_json());
    data
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    info!("Session Post");
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

pub fn test(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let mut data = BTreeMap::new();
    data.insert("title".to_string(), "Wow that worked!".to_string());

    resp.set_mut(Template::with("<h1>{{title}}</h1>", data)).set_mut(status::Ok);
    Ok(resp)
}

pub fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let mut data = make_data();
    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}
