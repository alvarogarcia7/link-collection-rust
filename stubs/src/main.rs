use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use serde_json::json;
use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use log::{info, warn, error};

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut port = 8181;
    let mut root_dir = PathBuf::from(".");

    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-p" | "--port" => {
                if i + 1 < args.len() {
                    port = args[i + 1].parse().unwrap_or(8181);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            _ => {
                if !args[i].starts_with('-') {
                    root_dir = PathBuf::from(&args[i]);
                }
                i += 1;
            }
        }
    }

    let root_dir = Arc::new(root_dir);
    let addr = ([0, 0, 0, 0], port).into();

    info!("Starting HTTP stub server on {} with root directory: {:?}", addr, root_dir);

    let make_service = make_service_fn(move |_conn| {
        let root_dir = Arc::clone(&root_dir);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let root_dir = Arc::clone(&root_dir);
                handle_request(req, root_dir)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
}

async fn handle_request(
    req: Request<Body>,
    root_dir: Arc<PathBuf>,
) -> Result<Response<Body>, Infallible> {
    let method = req.method().as_str();
    let path = req.uri().path();

    info!("Received {} request: {}", method, path);

    // Construct the directory path by removing leading slash
    let path_without_leading_slash = if path.starts_with('/') {
        &path[1..]
    } else {
        path
    };

    let endpoint_dir = root_dir.join(path_without_leading_slash);

    // Look for the $METHOD.json configuration file
    let config_filename = format!("${}.json", method);
    let config_path = endpoint_dir.join(&config_filename);

    info!("Looking for config: {:?}", config_path);

    match std::fs::read_to_string(&config_path) {
        Ok(config_content) => {
            match serde_json::from_str::<serde_json::Value>(&config_content) {
                Ok(config) => {
                    // Try to get the "default" response configuration
                    if let Some(default_response) = config.get("default") {
                        if let Some(body_file) = default_response.get("body").and_then(|v| v.as_str()) {
                            let status_code = default_response
                                .get("status")
                                .and_then(|v| v.as_u64())
                                .unwrap_or(200) as u16;

                            let response_file = endpoint_dir.join(body_file);
                            info!("Loading response from: {:?}", response_file);

                            match std::fs::read_to_string(&response_file) {
                                Ok(body) => {
                                    let status = StatusCode::from_u16(status_code)
                                        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

                                    info!("Returning status {} with body from {}", status_code, body_file);
                                    Ok(Response::builder()
                                        .status(status)
                                        .header("Content-Type", "application/json")
                                        .body(Body::from(body))
                                        .unwrap())
                                }
                                Err(e) => {
                                    warn!("Failed to read response file {:?}: {}", response_file, e);
                                    error_response(
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        &format!("Failed to read response file: {}", e),
                                    )
                                }
                            }
                        } else {
                            warn!("No 'body' field in default response configuration");
                            error_response(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Invalid configuration: missing 'body' field",
                            )
                        }
                    } else {
                        warn!("No 'default' response in configuration");
                        error_response(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Invalid configuration: missing 'default' response",
                        )
                    }
                }
                Err(e) => {
                    warn!("Failed to parse config JSON: {}", e);
                    error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &format!("Failed to parse configuration: {}", e),
                    )
                }
            }
        }
        Err(e) => {
            warn!("Configuration file not found: {:?}", config_path);
            error_response(
                StatusCode::NOT_FOUND,
                &format!("No stub found for {} {}", method, path),
            )
        }
    }
}

fn error_response(
    status: StatusCode,
    message: &str,
) -> Result<Response<Body>, Infallible> {
    let body = json!({
        "error": message
    }).to_string();

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Body::from(body))
        .unwrap())
}
