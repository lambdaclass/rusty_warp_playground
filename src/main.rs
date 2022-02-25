use rocksdb::DB;
use std::collections::HashMap;
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /shorten?url=<url>
    let shorten = warp::get()
        .and(warp::path("shorten"))
        .and(warp::query::<HashMap<String, String>>().map(|query: HashMap<String, String>| {
            let url = query["url"].clone();
            let storage = DB::open_default("rocks.db").unwrap();
            // TODO: Wrap url.clone() with actual shorten_link function
            let shortened_url = url.clone();
            storage.put(&shortened_url, &url).expect("Failed to insert");
            println!("{:?}", storage);
            shortened_url
        }));

    warp::serve(shorten).run(([127, 0, 0, 1], 8080)).await;
}
