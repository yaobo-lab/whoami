use std::{
    env,
    net::{SocketAddr, ToSocketAddrs},
    process::Command,
    time::Duration,
};

use axum::{
    body::{to_bytes, Body},
    extract::{ConnectInfo, State},
    http::{header, HeaderValue, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use network::utils;
use socket2::{Domain, Protocol, Socket, TcpKeepalive, Type};
use toolkit_rs::get_local_time;
use toolkit_rs::logger::{self, LogConfig, LogStyle};
use toolkit_rs::painc::{set_panic_handler, PaincConf};

#[derive(Clone)]
struct AppState {
    hostname: String,
    ips: Vec<String>,
}

async fn health() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], "OK")
}

async fn whoami(State(state): State<AppState>, request: Request<Body>) -> Response {
    let local_time = get_local_time();
    let (parts, request_body) = request.into_parts();
    if is_static_asset_path(parts.uri.path()) {
        return StatusCode::NO_CONTENT.into_response();
    }

    let addr = parts
        .extensions
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(addr)| addr.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let mut body = String::new();

    body.push_str(&format!("\nHostname: {}\n", state.hostname));
    body.push_str(&format!("LocalTime: {} \n", local_time));
    body.push('\n');

    for ip in &state.ips {
        body.push_str(&format!("IP: {ip}\n"));
    }
    body.push('\n');
    body.push_str(&format!("RemoteAddr: {addr}\n"));
    body.push('\n');
    body.push_str(&format!(
        "{} {} {:?}\n",
        parts.method, parts.uri, parts.version
    ));

    body.push_str("\nHeaders:\n");
    for (name, value) in parts.headers.iter() {
        body.push_str(&format!(
            "{}: {}\n",
            format_header_name(name.as_str()),
            header_value_to_string(value)
        ));
    }

    match to_bytes(request_body, usize::MAX).await {
        Ok(bytes) if !bytes.is_empty() => {
            body.push('\n');
            body.push_str("Body:\n");
            body.push_str(&String::from_utf8_lossy(&bytes));
        }
        Ok(_) => {}
        Err(err) => {
            body.push('\n');
            body.push_str(&format!("Body: <failed to read body: {err}>\n"));
        }
    }

    if !body.is_empty() {
        log::info!("{} \n\n", body);
    }
    ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], body).into_response()
}

fn is_static_asset_path(path: &str) -> bool {
    const STATIC_PATHS: &[&str] = &[
        "/favicon.ico",
        "/robots.txt",
        "/sitemap.xml",
        "/manifest.json",
        "/apple-touch-icon.png",
        "/apple-touch-icon-precomposed.png",
    ];

    const STATIC_EXTENSIONS: &[&str] = &[
        ".css", ".js", ".map", ".png", ".jpg", ".jpeg", ".gif", ".svg", ".webp", ".ico", ".bmp",
        ".txt", ".woff", ".woff2", ".ttf", ".eot",
    ];

    STATIC_PATHS.contains(&path) || STATIC_EXTENSIONS.iter().any(|ext| path.ends_with(ext))
}

fn hostname() -> String {
    env::var("HOSTNAME")
        .or_else(|_| env::var("COMPUTERNAME"))
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| command_output("hostname"))
        .unwrap_or_else(|| "unknown".to_string())
}

fn command_output(command: &str) -> Option<String> {
    let mut parts = command.split_whitespace();
    let program = parts.next()?;
    let output = Command::new(program).args(parts).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let value = String::from_utf8(output.stdout).ok()?.trim().to_string();
    (!value.is_empty()).then_some(value)
}

fn format_header_name(name: &str) -> String {
    name.split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    first.to_ascii_uppercase().to_string() + &chars.as_str().to_ascii_lowercase()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join("-")
}

fn header_value_to_string(value: &HeaderValue) -> String {
    value
        .to_str()
        .map(ToString::to_string)
        .unwrap_or_else(|_| "<non-utf8>".to_string())
}

fn bind_listener(host: &str, port: u16) -> anyhow::Result<tokio::net::TcpListener> {
    let addr = (host, port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| anyhow::anyhow!("failed to resolve bind address {host}:{port}"))?;

    let domain = if addr.is_ipv4() {
        Domain::IPV4
    } else {
        Domain::IPV6
    };
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_address(true)?;
    #[cfg(not(windows))]
    socket.set_reuse_port(true)?;
    socket.set_tcp_nodelay(true)?;
    socket.set_keepalive(true)?;
    socket.set_tcp_keepalive(
        &TcpKeepalive::new()
            .with_time(Duration::from_secs(60))
            .with_interval(Duration::from_secs(10)),
    )?;
    socket.set_nonblocking(true)?;
    socket.bind(&addr.into())?;
    socket.listen(2048)?;
    Ok(tokio::net::TcpListener::from_std(socket.into())?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    set_panic_handler(PaincConf::default());
    let mut cfg = LogConfig::default();
    cfg.level = 4;
    cfg.style = LogStyle::Module;
    cfg.size_mb = 10;
    cfg.keep_file_count = Some(5);
    cfg.console = true;
    logger::setup(cfg).expect("init logger failed");

    let ips = utils::get_local_ips().expect("get local ip failed");

    let state = AppState {
        hostname: hostname(),
        ips: ips,
    };

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/", get(whoami).post(whoami))
        .route("/{*path}", get(whoami).post(whoami))
        .with_state(state);

    //app = app.layer(TraceLayer::new_for_http());

    let listener = bind_listener("0.0.0.0", 3000)?;

    log::info!(
        "server is running on {}",
        listener
            .local_addr()
            .expect("get local addr failed")
            .to_string()
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
