mod configuration;
pub use configuration::*;

use axum::http::Uri;

#[derive(Clone)]
pub struct Endpoints {
    pub base_uri: Uri,
}

impl Endpoints {
    pub fn new(base_uri: Uri) -> Self {
        Endpoints { base_uri }
    }
}
