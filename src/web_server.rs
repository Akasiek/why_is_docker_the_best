use axum::Router;
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::routing::get;
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};
use crate::handlers::index;

pub async fn init() {
    let router = get_app_router();
    let listener = get_app_listener().await;

    info!(
        "Server listening on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, router).await.unwrap();
}

pub async fn get_app_listener() -> TcpListener {
    let mut listenfd = ListenFd::from_env();
    match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            info!("Using listener from environment (e.g., systemd socket activation)");
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => {
            let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
            TcpListener::bind(host).await.unwrap()
        }
    }
}

fn get_app_router() -> Router {
    Router::new()
        .route("/", get(index))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
}
