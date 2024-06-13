mod endpoints;

use axum::routing::get;
use axum::{Router};
use axum::http::Uri;
use tokio::net::TcpListener;
use crate::server::endpoints::{Endpoints, open_id_configuration_handler};

pub struct ServerApp {
    endpoints: Endpoints
}

impl ServerApp {
    pub fn new(baseUri: Uri) -> Self {
        ServerApp {
            endpoints: Endpoints::new(baseUri)
        }
    }

    pub async fn run(&self) {
        let app = Router::new()
            .route("/.well-known/openid-configuration", get(open_id_configuration_handler))
            //.route("/.well-known/openid-configuration/jwks", get(jwks_url))
            .with_state(self.endpoints.clone());

        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

        axum::serve(listener, app) .await.unwrap();
    }
}