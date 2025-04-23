use axum::{Router, extract::Request, routing::any};

pub async fn start_web_server() {
    tracing_subscriber::fmt().init();

    let rev_proxy = axum_reverse_proxy::ReverseProxy::new("/", "http://localhost:5173");

    let app = Router::new()
        .route("/api/ping", any(reply_200))
        .route("/api/ws", any(serve_ws))
        .merge(rev_proxy);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn reply_200() -> &'static str {
    "OK"
}

async fn serve_ws() {}

async fn redirect(req: Request) {}
