use std::{net::SocketAddrV4, path::PathBuf, sync::Arc};

use anyhow::bail;
use axum::{
    Router,
    extract::{Request, State},
    routing::any,
};
use tower_http::services::{ServeDir, ServeFile};
use tracing::info;

#[derive(Debug, clap::Parser)]
pub struct ServerCommand {
    #[clap(short, long, default_value = "3030")]
    port: u16,

    /// Reverse proxy a different server when path doesn't match any route.
    /// This is useful when developing the playground web UI and this server
    /// at the same time.
    #[clap(long, conflicts_with("serve_dir"))]
    reverse_proxy: Option<String>,

    /// Serve the web UI from a directory. Should be used when you only want to
    /// develop the font and don't care about the playground itself.
    #[clap(long, conflicts_with("reverse_proxy"))]
    serve_dir: Option<String>,
}

pub async fn start_web_server(cmd: ServerCommand) -> anyhow::Result<()> {
    let mut app = Router::new()
        .route("/api/ping", any(reply_200))
        .route("/api/ws", any(serve_ws));

    if let Some(url) = &cmd.reverse_proxy {
        info!("Reverse proxying to {}", url);
        let rev_proxy = axum_reverse_proxy::ReverseProxy::new("/", url);
        app = app.merge(rev_proxy);
    } else if let Some(dir) = &cmd.serve_dir {
        let dir = PathBuf::from(dir);
        if !dir.exists() {
            bail!("Served directory {} does not exist", dir.display());
        }
        info!("Serving directory {} as fallback", dir.display());
        let serve_dir =
            ServeDir::new(&dir).not_found_service(ServeFile::new(dir.join("index.html")));

        app = app.fallback_service(serve_dir);
    } else {
        info!("No reverse proxy or serve dir specified, no fallback service will be used.");
    }

    let listener =
        tokio::net::TcpListener::bind(SocketAddrV4::new([127, 0, 0, 1].into(), cmd.port))
            .await
            .unwrap();

    info!("Listening on {}", listener.local_addr().unwrap());

    let app = app.with_state(Arc::new(AppState {}));

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Debug, Clone)]
pub struct AppState {}

/// Extracted app state from the request.
type XAppState = State<Arc<AppState>>;

async fn reply_200() -> &'static str {
    "OK"
}

async fn serve_ws(State(state): State<Arc<AppState>>) {}

async fn redirect(req: Request) {}
