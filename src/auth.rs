use crate::config::AppState;
use axum::{
    body::Body,
    extract::{State, TypedHeader},
    headers::{authorization::Basic, Authorization},
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use std::sync::Arc;

macro_rules! unauthorized {
    () => {
        let resp = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(header::WWW_AUTHENTICATE, "Basic realm=\"Secure Area\"")
            .body(axum::body::boxed(Body::empty()))
            .unwrap();
        return resp;
    };
}

pub async fn auth_middleware<B, G>(
    State(state): State<Arc<AppState<G>>>,
    req: Request<B>,
    next: Next<B>,
) -> Response {
    let (mut parts, body) = req.into_parts();

    let auth = parts.extract::<TypedHeader<Authorization<Basic>>>().await;
    match auth {
        Ok(auth) => {
            if auth.username() != state.config.basic.user_name
                || auth.password() != state.config.basic.password
            {
                // invalid user
                unauthorized!();
            }
        }
        Err(e) => {
            // no auth header
            eprintln!("{}", e);
            unauthorized!();
        }
    }

    let req = Request::from_parts(parts, body);
    next.run(req).await
}
