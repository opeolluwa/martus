#[macro_use]
extern crate lazy_static;

use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;

use hyper::body::Bytes;
use hyper::{Body, Client, Request, Response, StatusCode, Uri};
use hyper::{HeaderMap, Method};

use std::collections::HashMap;
use std::fmt;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

// register the services
lazy_static! {
    static ref PROXY_SERVER: String = ServicePath::from_env("PROXY_SERVER", "http://0.0.0.0:5000");
    static ref STUDENT_SERVICE: String =
        ServicePath::from_env("MARTUS_STUDENT_SERVICE", "http://0.0.0.0:5001");
    static ref STAFF_SERVICE: String =
        ServicePath::from_env("MARTUS_STAFF_SERVICE", "http://0.0.0.0:5001");
    static ref LIBRARY_SERVICE: String =
        ServicePath::from_env("MARTUS_LIBRARY_SERVICE", "http://0.0.0.0:5003");
    static ref HOSTEL_SERVICE: String =
        ServicePath::from_env("MARTUS_HOSTEL_SERVICE", "http://0.0.0.0:5004");
    pub static ref REGISTERED_SERVICES: HashMap<String, String> = HashMap::from([
        ServicePath::from("student", STUDENT_SERVICE.to_string()),
        ServicePath::from("staff", STAFF_SERVICE.to_string()),
        ServicePath::from("library", LIBRARY_SERVICE.to_string()),
        ServicePath::from("hostel", HOSTEL_SERVICE.to_string()),
    ]);
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    // load env variables
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "martus-proxy=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init(); // allow debugging in development set up

    // define cors scope as any
    let cors_layer = CorsLayer::new().allow_headers(Any).allow_methods([
        Method::GET,
        Method::POST,
        Method::DELETE,
        Method::PUT,
        Method::PATCH,
    ]); // restrict methods

    // build our application with a route to match all HTTP verbs
    let app = Router::new()
        .route(
            "/v1/*path", // match all request method
            post(handler)
                .get(handler)
                .patch(handler)
                .put(handler)
                .delete(handler),
        )
        .route("/health", get(health_check))
        .layer(cors_layer)
        .fallback(handle_404);

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}

#[derive(Debug, Default)]
struct Proxy {
    pub headers: HeaderMap,
    pub method: Method,
    pub path: Uri,
    pub body: Body,
}

// impl display
impl fmt::Display for Proxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "headers {:#?}\nmethod: {}\npath: {}\nbody:{:#?}\n",
            self.headers, self.method, self.path, self.body
        )
    }
}

struct ServicePath;

#[allow(dead_code)]
impl ServicePath {
    // create a new service
    pub fn from(service_id: &str, service_base_url: String) -> (String, String) {
        let service_id = service_id.to_string();
        let service_base_url = service_base_url.to_string();

        (service_id, service_base_url)
    }

    // parse the url
    pub fn parse_url(path: Uri) -> String {
        let path = path.path().split('/').collect::<Vec<&str>>();

        // detect the recipient server
        let service_id = path[2];
        let service_base_url = REGISTERED_SERVICES.get(service_id).unwrap_or(&PROXY_SERVER); // SERVING THE REQUEST TO THE PROXY SERVER WOULD RETURN A 404 ERROR SINCE NO ROUTE WOULD BE MATCHED
        let resource_path = &path[3..].join("/");

        let url = format!("{service_base_url}{resource_path}");

        url
    }
    // read the url from env
    fn from_env<'a>(key: &'a str, default: &'a str) -> std::string::String {
        std::env::var(key).unwrap_or(default.to_string())
    }
}

/// 404 handler
async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        axum::response::Json(serde_json::json!({
        "success":false,
        "message":String::from("The requested resource does not exist on this server!"),
        })),
    )
}

/// health check
async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        axum::response::Json(serde_json::json!({
        "success":true,
        "message":String::from("The server is up and running!"),
        })),
    )
}

// `Request` gives you the whole request for maximum control
async fn handler(path: Uri, method: Method, headers: HeaderMap, body: Bytes) -> Response<Body> {
    // pass data to request builder
    let body = Body::from(body);
    let url = ServicePath::parse_url(path);
    let mut req = Request::builder();

    // add the header to the built request object
    for (key, value) in headers {
        req.headers_mut().unwrap().insert(key.unwrap(), value);
    }

    let req = req.method(method).uri(&url).body(body).unwrap();
    let client = Client::new();
    let res = client.request(req).await;

    // map response to return type
    //TODO: improve error handling
    res.unwrap()
}