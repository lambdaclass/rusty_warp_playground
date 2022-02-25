mod urlcreation;

use rocksdb::{Options, DB};
use std::sync::{Arc, Mutex};
use warp::{http::Uri, Filter};

use std::str::FromStr;

#[tokio::main]
async fn main() {
    let path = "rocks.db";

    {
        let db = DB::open_default(path).unwrap();
        db.put(b"191347bfe5", b"http://www.google.com").unwrap();

        let api = filters::endpoints(Arc::new(Mutex::new(db)));
        warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
    }
    let _ = DB::destroy(&Options::default(), path);
}

mod filters {
    use super::*;

    pub fn endpoints(db: Arc<Mutex<DB>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        shortened_url(db.clone())
    }

    pub fn shortened_url(db: Arc<Mutex<DB>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!(String).map(move |shortened_url: String| {
            println!("url: {}", shortened_url);
            let new_url = match (*db).lock().unwrap().get(shortened_url) {
                Ok(Some(value)) => std::str::from_utf8(&value[..]).unwrap().to_string(),
                _ => "did not find".to_string(),
            };

            let location = Uri::from_str(&new_url).unwrap();
            warp::redirect(location)
        })
    }
}
