pub fn construct_localhost_url(query: &str) -> String {
    // default
    if query == "lh" {
        let localhost = "http://localhost:3000";

        localhost.to_string()
    } else {
        // Assume the other match is "localhost:PORT"
        // optimistic, eh?
        let port = &query[3..];
        let localhost_url = format!("http://localhost:{}", port);

        localhost_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_localhost_url_with_gh() {
        let fake_query = "lh";
        assert_eq!(construct_localhost_url(fake_query), "http://localhost:3000");
    }

    #[test]
    fn test_construct_localhost_url_with_repo_url() {
        let fake_query = "lh 8080";
        assert_eq!(
            construct_localhost_url(fake_query),
            "http://localhost:8080"
        );
    }
}
