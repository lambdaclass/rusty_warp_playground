use warp::Filter;

#[tokio::main]
async fn main() {
    //pretty_env_logger::init();
    
    // GET /ping
    let ping = warp::path("ping").map(|| "Pong!");

    let routes = warp::get().and(
        ping,
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
