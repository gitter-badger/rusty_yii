use iron::Handler;
use iron::status;
use iron::prelude::*;
use std::collections::HashMap;
pub struct Router {
    routes: HashMap<String, Box<Handler>>,
}
impl Router {
    pub fn new() -> Router {
        Router { routes: HashMap::new() }
    }
    pub fn add_route<H>(&mut self, path: String, handler: H)
        where H: Handler
    {
        self.routes.insert(path, Box::new(handler));
    }
}
impl Handler for Router {
     fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path.join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}