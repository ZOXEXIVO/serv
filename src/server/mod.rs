mod endpoints;

use axum::routing::get;
use axum::{Router};
use axum::http::Uri;
use tokio::net::TcpListener;
use crate::server::endpoints::{Endpoints, jwks_handler, openid_configuration_handler};

pub struct ServerApp {
    endpoints: Endpoints
}

impl ServerApp {
    pub fn new(base_uri: Uri) -> Self {
        ServerApp {
            endpoints: Endpoints::new(base_uri)
        }
    }

    pub async fn run(&self) {
        let app = Router::new()
            .route("/.well-known/openid-configuration", get(openid_configuration_handler))
            .route("/.well-known/openid-configuration/jwks", get(jwks_handler))
            .with_state(self.endpoints.clone());

        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

        axum::serve(listener, app) .await.unwrap();
    }
}