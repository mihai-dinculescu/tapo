use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use subtle::ConstantTimeEq;

const WWW_AUTHENTICATE: &str = "WWW-Authenticate";
const BEARER: &str = "Bearer";

pub async fn require_bearer_token(
    State(expected_key): State<String>,
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    let token = extract_bearer_token(request.headers()).ok_or_else(unauthorized)?;

    if expected_key.as_bytes().ct_eq(token.as_bytes()).unwrap_u8() != 1 {
        return Err(unauthorized());
    }

    Ok(next.run(request).await)
}

fn unauthorized() -> Response {
    (StatusCode::UNAUTHORIZED, [(WWW_AUTHENTICATE, BEARER)]).into_response()
}

fn extract_bearer_token(headers: &axum::http::HeaderMap) -> Option<&str> {
    let value = headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;

    // RFC 7235: auth-scheme is case-insensitive, followed by 1*SP.
    let scheme = value.get(..BEARER.len())?;
    if !scheme.eq_ignore_ascii_case(BEARER) {
        return None;
    }

    let rest = &value[BEARER.len()..];
    let token = rest.trim_start();

    // At least one whitespace character must separate scheme from token,
    // and the token itself must be non-empty.
    if rest.len() == token.len() || token.is_empty() {
        return None;
    }

    Some(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[test]
    fn extract_valid() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer my-secret-key".parse().unwrap());
        assert_eq!(extract_bearer_token(&headers), Some("my-secret-key"));
    }

    #[test]
    fn extract_case_insensitive_scheme() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "bearer my-secret-key".parse().unwrap());
        assert_eq!(extract_bearer_token(&headers), Some("my-secret-key"));
    }

    #[test]
    fn extract_extra_whitespace() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer   my-secret-key".parse().unwrap());
        assert_eq!(extract_bearer_token(&headers), Some("my-secret-key"));
    }

    #[test]
    fn extract_missing_header() {
        let headers = HeaderMap::new();
        assert_eq!(extract_bearer_token(&headers), None);
    }

    #[test]
    fn extract_wrong_scheme() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Basic dXNlcjpwYXNz".parse().unwrap());
        assert_eq!(extract_bearer_token(&headers), None);
    }

    #[test]
    fn extract_whitespace_only_token() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer   ".parse().unwrap());
        assert_eq!(extract_bearer_token(&headers), None);
    }

    #[test]
    fn extract_no_space_after_bearer() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "BearerNoSpace".parse().unwrap());
        assert_eq!(extract_bearer_token(&headers), None);
    }
}
