use iron::prelude::*;
use iron::modifiers::Redirect;
use iron::{Url, status};
use urlencoded::{UrlEncodedQuery, UrlDecodingError};

const ERR_MISSING_URL: &'static str = "Missing 'url' query parameter!";
const ERR_PARSE_FAILURE: &'static str = "Failed to parse query string!";

pub fn bounce_handler(req: &mut Request) -> IronResult<Response> {
    match req.get_ref::<UrlEncodedQuery>() {
        // Get the query string, parse out hashmap, get the "url" param
        Ok(ref hashmap) => {
            hashmap.get("url")
            // "url" missing, convert None to Err
            .ok_or("missing url error".to_owned())
            // Found "url" now parse it as a Url
            .and_then(|url| Url::parse(url[0].as_str()))
            // Redirect to the parsed Url
            .and_then(|url| {
                info!("Bounce to {:?}", url);
                Ok(Response::with((status::Found, Redirect(url.clone()))))
            })
            .or_else(|_| {
                error!("{}", ERR_MISSING_URL);
                Ok(Response::with((status::BadRequest, ERR_MISSING_URL)))
            })
        }
        Err(UrlDecodingError::EmptyQuery) => {
            Ok(Response::with((status::BadRequest, ERR_MISSING_URL)))
        }
        Err(_) => Ok(Response::with((status::InternalServerError, ERR_PARSE_FAILURE))),
    }
}
