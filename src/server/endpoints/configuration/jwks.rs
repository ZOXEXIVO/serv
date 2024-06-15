use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;

pub async fn jwks_handler() -> impl IntoResponse {
    let jwks_config = JwksConfiguration {
        keys: vec![
            JsonWebKey {
                kty: "RSA".to_string(),
                kid: "1".to_string(),
                use_: "sig".to_string(),
                alg: "RS256".to_string(),
                n: "base64url-encoded-modulus".to_string(),
                e: "base64url-encoded-exponent".to_string(),
            },
            // Add more keys if needed
        ],
    };

    let mut headers = HeaderMap::new();

    headers.insert("ContentType", "application/json".parse().unwrap());

    (StatusCode::OK, headers, Json(jwks_config))
}

#[derive(Serialize)]
pub struct JwksConfiguration{
    keys: Vec<JsonWebKey>,
}

#[derive(Serialize)]
pub struct JsonWebKey {
    kty: String, // Key Type, e.g., "RSA"
    kid: String, // Key ID
    #[serde(rename = "use")]
    use_: String, // Public Key Use, e.g., "sig"
    alg: String, // Algorithm, e.g., "RS256"
    n: String, // Modulus for RSA keys
    e: String, // Exponent for RSA keys
}