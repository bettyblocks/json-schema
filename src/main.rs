use axum::body::Body;
use axum::extract::Request;
use axum::http::uri::Scheme;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use once_cell::sync::Lazy;
use regex::Regex;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceExt;
use tower_http::services::fs::ServeFileSystemResponseBody;
use tower_http::services::{ServeDir, ServeFile};

const PORT: u16 = 9797;
const SCHEMA: &str = "http://json-schema.org/draft-07/schema";
const HOST: &str = "https://raw.githubusercontent.com";
const FALLBACK_HOST_HEADER: HeaderValue = HeaderValue::from_static("example.com");
static REGEX_JSON_SCHEMA: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"bettyblocks/json-schema/\w+/").unwrap());

enum SchemaOrDir {
    Schema,
    Dir,
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route(
            "/schema",
            get(|request| fetch_file(request, SchemaOrDir::Schema)),
        )
        .nest_service("/", get(|request| fetch_file(request, SchemaOrDir::Dir)));

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn fetch_file(
    request: Request,
    schema_or_dir: SchemaOrDir,
) -> Result<impl IntoResponse, String> {
    let hostname = resolve_hostname(&request);

    let result = match schema_or_dir {
        SchemaOrDir::Schema => {
            let service = ServeFile::new("schema.json");
            service.oneshot(request).await
        }
        SchemaOrDir::Dir => {
            let service = ServeDir::new("");
            service.oneshot(request).await
        }
    };

    match result {
        Ok(response) => fetch_and_convert_file(response, &hostname).await,
        Err(e) => Err(e.to_string()),
    }
}

fn resolve_hostname(request: &Request) -> String {
    let fallback_header = FALLBACK_HOST_HEADER;
    let host_header = request.headers().get("host").unwrap_or(&fallback_header);
    let host_header_str = host_header.to_str().expect("host should be a string");

    let protocol = if host_header_str.contains("localhost") {
        request.uri().scheme().unwrap_or(&Scheme::HTTP)
    } else {
        &Scheme::HTTPS
    };

    format!("{}://{}", protocol, host_header_str)
}

async fn fetch_and_convert_file(
    response: Response<ServeFileSystemResponseBody>,
    hostname: &str,
) -> Result<impl IntoResponse, String> {
    let body = Body::new(response);

    let data = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    let txt = std::str::from_utf8(&data).unwrap();

    let new_text = txt.replace(HOST, &hostname);
    let new_text = REGEX_JSON_SCHEMA.replace_all(&new_text, "");
    let new_text = new_text.replace(SCHEMA, &format!("{hostname}/schema"));
    Ok((
        StatusCode::OK,
        [("content-type", "application/json")],
        new_text,
    ))
}
