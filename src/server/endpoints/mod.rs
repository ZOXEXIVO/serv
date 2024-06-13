mod openid_configuration;

use axum::http::Uri;
pub use openid_configuration::*;

#[derive(Clone)]
pub struct Endpoints {
    pub baseUri: Uri
}

impl Endpoints {
    pub fn new(baseUri: Uri) -> Self {
        Endpoints {
            baseUri
        }
    }
}