use crate::server::endpoints::Endpoints;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;

pub async fn openid_configuration_handler(endpoints: State<Endpoints>) -> impl IntoResponse {
    let openid_config = OpenIdConfiguration {
        issuer: endpoints.base_uri.to_string(),
        authorization_endpoint: format!("{}authorize", &endpoints.base_uri),
        token_endpoint: format!("{}connect/token", &endpoints.base_uri),
        userinfo_endpoint: format!("{}userinfo", &endpoints.base_uri),
        jwks_uri: format!(
            "{}.well-known/openid-configuration/jws",
            &endpoints.base_uri
        ),
        registration_endpoint: None,
        scopes_supported: vec![
            "openid".to_string(),
            "profile".to_string(),
            "email".to_string(),
        ],
        response_types_supported: vec!["token".to_string()],
        subject_types_supported: vec![],
        claims_supported: vec!["sub".to_string()],
    };

    let mut headers = HeaderMap::new();

    headers.insert("ContentType", "application/json".parse().unwrap());

    (StatusCode::OK, headers, Json(openid_config))
}

#[derive(Serialize)]
pub struct OpenIdConfiguration {
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: String,
    jwks_uri: String,
    registration_endpoint: Option<String>,
    scopes_supported: Vec<String>,
    response_types_supported: Vec<String>,
    subject_types_supported: Vec<String>,
    claims_supported: Vec<String>,
}
