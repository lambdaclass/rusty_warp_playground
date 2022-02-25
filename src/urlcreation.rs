use sha2::{Digest, Sha256};

fn hash_url(url: String) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write url
    hasher.update(url.into_bytes());

    // read hash digest and consume hasher
    format!("{:?}", hasher.finalize())
}

pub fn shorten_url(url: String) -> String {
    let mut hashed_url = hash_url(url);
    hashed_url.truncate(10);
    hashed_url
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_url_hashed_correctly() {
        let url = "www.google.com".to_string();
        let expected_shortened_url = "191347bfe5".to_string();

        assert_eq!(expected_shortened_url, shorten_url(expected_shortened_url.to_string()));
    }
}
