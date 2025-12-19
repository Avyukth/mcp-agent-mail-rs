use axum::extract::ConnectInfo;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use governor::{Quota, RateLimiter, clock::DefaultClock, state::keyed::DashMapStateStore};
use std::net::SocketAddr;
use std::num::NonZeroU32;
use std::sync::Arc;
use tracing::{debug, warn};

/// Rate limiter keyed by composite identity string.
///
/// Key format: `{jwt_subject}:{ip}` for authenticated requests,
/// or just `{ip}` for unauthenticated requests.
///
/// NIST Control: SC-5 (DoS Protection)
type KeyedRateLimiter = RateLimiter<String, DashMapStateStore<String>, DefaultClock>;

#[derive(Clone)]
pub struct RateLimitConfig {
    pub limiter: Arc<KeyedRateLimiter>,
    pub enabled: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimitConfig {
    #[allow(clippy::expect_used)] // NonZeroU32 from parsed u32 with fallback defaults; always valid
    pub fn new() -> Self {
        let enabled =
            std::env::var("RATE_LIMIT_ENABLED").unwrap_or_else(|_| "true".into()) == "true";

        // Defaults sized for 100 concurrent agents:
        // - 1000 RPS allows 10 requests/second per agent
        // - 2000 burst handles initial connection spikes
        let rps = std::env::var("RATE_LIMIT_RPS")
            .unwrap_or_else(|_| "1000".into())
            .parse::<u32>()
            .unwrap_or(1000);

        let burst = std::env::var("RATE_LIMIT_BURST")
            .unwrap_or_else(|_| "2000".into())
            .parse::<u32>()
            .unwrap_or(2000);

        let quota = Quota::per_second(NonZeroU32::new(rps).expect("RPS should be non-zero"))
            .allow_burst(NonZeroU32::new(burst).expect("Burst should be non-zero"));

        let limiter = Arc::new(RateLimiter::keyed(quota));

        tracing::info!(
            "Rate Limiting: enabled={}, rps={}, burst={}",
            enabled,
            rps,
            burst
        );

        Self { limiter, enabled }
    }
}

/// Extract JWT subject from Authorization header without full verification.
///
/// This only decodes the JWT payload to extract the `sub` claim for rate limiting.
/// Authentication and signature verification should be done by the auth middleware.
///
/// # Arguments
/// * `auth_header` - The Authorization header value (e.g., "Bearer eyJ...")
///
/// # Returns
/// The `sub` claim if present and valid, or `None`
fn extract_jwt_subject(auth_header: &str) -> Option<String> {
    // Extract token from "Bearer <token>"
    let token = auth_header.strip_prefix("Bearer ")?.trim();

    // JWT has 3 parts: header.payload.signature
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }

    // Decode the payload (second part)
    let payload = URL_SAFE_NO_PAD.decode(parts[1]).ok().or_else(|| {
        // Try with padding
        let padded = format!("{}{}", parts[1], "=".repeat((4 - parts[1].len() % 4) % 4));
        base64::engine::general_purpose::URL_SAFE
            .decode(&padded)
            .ok()
    })?;

    // Parse as JSON and extract "sub"
    let claims: serde_json::Value = serde_json::from_slice(&payload).ok()?;
    claims.get("sub")?.as_str().map(|s| s.to_string())
}

/// Construct the rate limit bucket key.
///
/// Key format:
/// - `{jwt_subject}:{ip}` for authenticated requests
/// - `{ip}` for unauthenticated requests
///
/// NIST Control: SC-5 (DoS Protection)
pub fn get_bucket_key(req: &Request, client_ip: std::net::IpAddr) -> String {
    // Try to extract JWT subject from Authorization header
    if let Some(auth_header) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(subject) = extract_jwt_subject(auth_str) {
                debug!(
                    subject = %subject,
                    ip = %client_ip,
                    "Rate limit key includes JWT subject"
                );
                return format!("{}:{}", subject, client_ip);
            }
        }
    }

    // Fallback to IP-only for unauthenticated requests
    client_ip.to_string()
}

pub async fn rate_limit_middleware(
    State(config): State<RateLimitConfig>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if !config.enabled {
        return Ok(next.run(req).await);
    }

    // Determine Client IP
    // Prefer X-Forwarded-For header if present (standard for reverse proxies)
    // Fallback to direct peer address (ConnectInfo)
    let ip = if let Some(forwarded) = req.headers().get("x-forwarded-for") {
        forwarded
            .to_str()
            .ok()
            .and_then(|s| s.split(',').next()) // Take the first IP in the list
            .and_then(|s| s.trim().parse::<std::net::IpAddr>().ok())
            .unwrap_or(peer.ip())
    } else {
        peer.ip()
    };

    // Get bucket key (includes JWT subject if present)
    let bucket_key = get_bucket_key(&req, ip);

    match config.limiter.check_key(&bucket_key) {
        Ok(_) => Ok(next.run(req).await),
        Err(_) => {
            warn!(bucket_key = %bucket_key, "RateLimit: exceeded quota");
            Err(StatusCode::TOO_MANY_REQUESTS)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request as HttpRequest;
    use std::net::IpAddr;

    /// Create a test JWT token with given subject.
    /// Format: header.payload.signature (signature is fake for testing)
    fn create_test_jwt(subject: &str) -> String {
        use base64::engine::general_purpose::URL_SAFE_NO_PAD;

        // Minimal header
        let header = r#"{"alg":"HS256","typ":"JWT"}"#;
        let header_b64 = URL_SAFE_NO_PAD.encode(header);

        // Payload with sub claim
        let payload = format!(r#"{{"sub":"{}","iat":1234567890}}"#, subject);
        let payload_b64 = URL_SAFE_NO_PAD.encode(payload);

        // Fake signature (not verified in rate limiting)
        let signature = "fake_signature_for_testing";

        format!("{}.{}.{}", header_b64, payload_b64, signature)
    }

    #[test]
    fn test_extract_jwt_subject_valid() {
        let token = create_test_jwt("user123");
        let auth_header = format!("Bearer {}", token);

        let subject = extract_jwt_subject(&auth_header);
        assert_eq!(subject, Some("user123".to_string()));
    }

    #[test]
    fn test_extract_jwt_subject_with_email() {
        let token = create_test_jwt("alice@example.com");
        let auth_header = format!("Bearer {}", token);

        let subject = extract_jwt_subject(&auth_header);
        assert_eq!(subject, Some("alice@example.com".to_string()));
    }

    #[test]
    fn test_extract_jwt_subject_missing_bearer() {
        let token = create_test_jwt("user123");
        // No "Bearer " prefix
        let subject = extract_jwt_subject(&token);
        assert_eq!(subject, None);
    }

    #[test]
    fn test_extract_jwt_subject_invalid_token_format() {
        // Only 2 parts instead of 3
        let subject = extract_jwt_subject("Bearer header.payload");
        assert_eq!(subject, None);
    }

    #[test]
    fn test_extract_jwt_subject_invalid_base64() {
        let subject = extract_jwt_subject("Bearer !!invalid!!.!!base64!!.signature");
        assert_eq!(subject, None);
    }

    #[test]
    fn test_extract_jwt_subject_no_sub_claim() {
        use base64::engine::general_purpose::URL_SAFE_NO_PAD;

        // JWT without sub claim
        let header = r#"{"alg":"HS256"}"#;
        let payload = r#"{"iat":1234567890}"#;
        let token = format!(
            "{}.{}.sig",
            URL_SAFE_NO_PAD.encode(header),
            URL_SAFE_NO_PAD.encode(payload)
        );

        let subject = extract_jwt_subject(&format!("Bearer {}", token));
        assert_eq!(subject, None);
    }

    #[test]
    fn test_get_bucket_key_with_jwt() {
        let token = create_test_jwt("agent-001");
        let ip: IpAddr = "192.168.1.100".parse().unwrap();

        let req = HttpRequest::builder()
            .header("authorization", format!("Bearer {}", token))
            .body(())
            .unwrap();

        // Convert to axum Request type
        let axum_req: Request = req.map(|_| axum::body::Body::empty());

        let key = get_bucket_key(&axum_req, ip);
        assert_eq!(key, "agent-001:192.168.1.100");
    }

    #[test]
    fn test_get_bucket_key_without_jwt() {
        let ip: IpAddr = "10.0.0.1".parse().unwrap();

        let req = HttpRequest::builder().body(()).unwrap();
        let axum_req: Request = req.map(|_| axum::body::Body::empty());

        let key = get_bucket_key(&axum_req, ip);
        assert_eq!(key, "10.0.0.1");
    }

    #[test]
    fn test_get_bucket_key_with_invalid_jwt_falls_back_to_ip() {
        let ip: IpAddr = "172.16.0.1".parse().unwrap();

        let req = HttpRequest::builder()
            .header("authorization", "Bearer invalid.token")
            .body(())
            .unwrap();
        let axum_req: Request = req.map(|_| axum::body::Body::empty());

        let key = get_bucket_key(&axum_req, ip);
        assert_eq!(key, "172.16.0.1");
    }

    #[test]
    fn test_get_bucket_key_ipv6() {
        let token = create_test_jwt("ipv6-user");
        let ip: IpAddr = "2001:db8::1".parse().unwrap();

        let req = HttpRequest::builder()
            .header("authorization", format!("Bearer {}", token))
            .body(())
            .unwrap();
        let axum_req: Request = req.map(|_| axum::body::Body::empty());

        let key = get_bucket_key(&axum_req, ip);
        assert_eq!(key, "ipv6-user:2001:db8::1");
    }

    #[test]
    fn test_rate_limit_config_defaults() {
        // Test that RateLimitConfig can be created with defaults
        let config = RateLimitConfig::new();
        assert!(config.enabled);
    }
}
