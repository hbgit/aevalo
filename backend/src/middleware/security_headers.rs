// middleware/security_headers.rs
//! Security headers middleware for HTTP responses

use axum::http::{HeaderName, HeaderValue};
use tower_http::set_header::SetResponseHeaderLayer;

/// Creates CSP header layer
pub fn csp_header_layer() -> SetResponseHeaderLayer<HeaderValue> {
    let csp = "default-src 'self'; \
               script-src 'self'; \
               connect-src 'self' https://*.supabase.co; \
               style-src 'self' https://fonts.googleapis.com; \
               img-src 'self' data: https:; \
               font-src 'self' https://fonts.gstatic.com; \
               frame-ancestors 'none'; \
               base-uri 'self'; \
               form-action 'self'";

    SetResponseHeaderLayer::overriding(
        HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static(csp),
    )
}

/// Creates X-Content-Type-Options header layer
pub fn x_content_type_options_layer() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    )
}

/// Creates X-Frame-Options header layer
pub fn x_frame_options_layer() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    )
}

/// Creates X-XSS-Protection header layer
pub fn x_xss_protection_layer() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("1; mode=block"),
    )
}

/// Creates HSTS header layer
pub fn hsts_header_layer() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(
        HeaderName::from_static("strict-transport-security"),
        HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
    )
}

/// Creates Referrer-Policy header layer
pub fn referrer_policy_layer() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(
        HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    )
}
