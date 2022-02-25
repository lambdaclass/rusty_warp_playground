use futures::future;
use warp::{hyper::Uri, Filter};

#[tokio::main]
async fn main() {
    //pretty_env_logger::init();

    let http_route = warp::path("ping").map(|| warp::redirect(Uri::from_static("https://127.0.0.1:8080")));

    let https_routes = warp::path("ping").map(|| "Connected through tls");
    let (_http_addr, http_warp) = warp::serve(http_route).bind_ephemeral(([127, 0, 0, 1], 3030));
    let (_https_addr, https_warp) = warp::serve(https_routes)
        .tls()
        .cert_path("tls/MyCertificate.crt")
        .key_path("tls/MyKey.key")
        .bind_ephemeral(([127, 0, 0, 1], 8080));

    future::join(http_warp, https_warp).await;
}
