use axum::body::{Body, StreamBody};
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tower_http::trace::TraceLayer;
use tracing::debug;
use tracing::Span;

pub(crate) async fn run_server(port: u16, resource_root: PathBuf, home_page: String) {
    let get_home = |State(resource_root): State<Arc<PathBuf>>| async {
        get_path(State(resource_root), home_page).await
    };
    // let get_interactive = |State(resource_root): State<Arc<PathBuf>>| async {
    //     get_path(State(resource_root), "interactive.html".to_string()).await
    // };
    // let get_resume = |State(resource_root): State<Arc<PathBuf>>| async {
    //     get_path(State(resource_root), "resume.html".to_string()).await
    // };
    let application = Router::new()
        .route("/", get(get_home))
        // .route("/interactive", get(get_interactive))
        // .route("/resume", get(get_resume))
        .route("/api", get(|| async { format!("Api!") }))
        .route("/api/do_stuff", get(|| async { format!("Api do_stuff!") }))
        .fallback(get_resource)
        .layer(
            TraceLayer::new_for_http().on_request(|request: &Request<Body>, _span: &Span| {
                tracing::info!("started {} {}", request.method(), request.uri().path())
            }),
        )
        .with_state(Arc::new(resource_root));

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    axum::Server::bind(&address)
        .serve(application.into_make_service())
        .await
        .unwrap();
}

async fn get_resource(
    State(resource_root): State<Arc<PathBuf>>,
    req: Request<Body>,
) -> Result<impl IntoResponse, StatusCode> {
    get_path(State(resource_root), req.uri().path().to_string()).await
}

async fn get_path(
    State(resource_root): State<Arc<PathBuf>>,
    path: String,
) -> Result<impl IntoResponse, StatusCode> {
    let path = PathBuf::try_from(path.trim_start_matches('/')).unwrap();
    let resource_path = resource_root.join(path);
    debug!("get_path({resource_path:?})");

    let file = match resource_path.canonicalize() {
        Ok(resource_canonical) => {
            // Extra guardrail - make sure the specified path doesn't walk "up" the directory tree.
            // The server framework already seems to strip any relative paths, but this would protect in case that ever goes wrong.
            if resource_path == resource_canonical {
                File::open(resource_canonical)
                    .await
                    .map_err(|_| StatusCode::NOT_FOUND)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }?;

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    Ok(body)
}
