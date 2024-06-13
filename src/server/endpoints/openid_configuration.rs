use axum::http::{StatusCode};
use axum::{Json};
use axum::extract::State;
use serde::Serialize;
use crate::server::endpoints::Endpoints;

pub async fn open_id_configuration_handler(State(endpoints): State<Endpoints>) -> (StatusCode, Json<OpenIdConfiguration>){
    let openid_config = OpenIdConfiguration {
        issuer: endpoints.baseUri.to_string(),
        authorization_endpoint: "https://example.com/authorize".to_string(),
        token_endpoint: "https://example.com/token".to_string(),
        userinfo_endpoint: "https://example.com/userinfo".to_string(),
        jwks_uri: "https://example.com/.well-known/jwks.json".to_string(),
        registration_endpoint: None,
        scopes_supported: vec![
            "openid".to_string(),
            "profile".to_string(),
            "email".to_string(),
        ],
        response_types_supported: vec![
            "code".to_string(),
            "token".to_string(),
            "id_token".to_string(),
            "code token".to_string(),
            "code id_token".to_string(),
            "token id_token".to_string(),
            "code token id_token".to_string(),
        ],
        subject_types_supported: vec!["public".to_string()],
        id_token_signing_alg_values_supported: vec!["RS256".to_string()],
        token_endpoint_auth_methods_supported: vec!["client_secret_basic".to_string()],
        claims_supported: vec![
            "sub".to_string(),
            "name".to_string(),
            "preferred_username".to_string(),
            "email".to_string(),
        ],
    };

    (StatusCode::OK, Json(openid_config))
}

#[derive(Serialize)]
pub struct OpenIdConfiguration{
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: String,
    jwks_uri: String,
    registration_endpoint: Option<String>,
    scopes_supported: Vec<String>,
    response_types_supported: Vec<String>,
    subject_types_supported: Vec<String>,
    id_token_signing_alg_values_supported: Vec<String>,
    token_endpoint_auth_methods_supported: Vec<String>,
    claims_supported: Vec<String>,
}