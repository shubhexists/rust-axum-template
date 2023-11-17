use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
mod error;
mod model;
mod web;
pub use self::error::{Error, Result};
use crate::model::ModelController;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

//In the parameters of this function, we are destructuring and deserealizing the params
//http://127.0.0.1:8080/hello?name=Shubham
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name: Option<&str> = params.name.as_deref();
    let name = match name {
        Some(_str) => name.unwrap(),
        None => "World!",
    };
    Html(format!("Hello {}", name))
}

//http://127.0.0.1:8080/hello/Shubham
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello {}", name))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    //We would usually point this to a HTML File
    //http://127.0.0.1:8080/src/main.rs
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("--> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}


#[tokio::main]
async fn main() -> Result<()> {
    let mc: ModelController = ModelController::new().await?;
    
    let routes_apis: Router = web::routes_tickets::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    //layers get executed from bottom to top
    let routes_all: Router = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis )
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("--> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    Ok(())
}
