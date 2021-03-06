//! Rewrite the request path to "index.html" if the path doesn't start
//! with "/api" and the Accept header contains "html".

use super::prelude::*;

use util::RequestProxy;

// Can't derive debug because of Handler and Static.
#[allow(missing_debug_implementations)]
pub struct EmberIndexRewrite {
    handler: Option<Box<Handler>>,
}

impl Default for EmberIndexRewrite {
    fn default() -> EmberIndexRewrite {
        EmberIndexRewrite { handler: None }
    }
}

impl AroundMiddleware for EmberIndexRewrite {
    fn with_handler(&mut self, handler: Box<Handler>) {
        self.handler = Some(handler);
    }
}

impl Handler for EmberIndexRewrite {
    fn call(&self, req: &mut Request) -> Result<Response, Box<Error + Send>> {
        // If the client is requesting html, then we've only got one page so
        // rewrite the request.
        let wants_html = req.headers()
            .find("Accept")
            .map(|accept| accept.iter().any(|s| s.contains("html")))
            .unwrap_or(false);
        // If the route starts with /api, just assume they want the API
        // response and fall through.
        let is_api_path = req.path().starts_with("/api");
        let handler = self.handler.as_ref().unwrap();
        if wants_html && !is_api_path {
            handler.call(&mut RequestProxy {
                other: req,
                path: Some("/index.html"),
                method: None,
            })
        } else {
            handler.call(req)
        }
    }
}
