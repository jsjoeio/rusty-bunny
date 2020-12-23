pub mod github;
pub mod google;
pub mod localhost;
pub mod twitter;

pub fn get_command_from_query_string(query_string: &str) -> &str {
    // If it has a space, we know that it is more than the command
    if query_string.contains(' ') {
        // We need to this to know where to slice the string
        // TODO add note about why we need the unrap_or (tbh i'm not sure why...)
        // copied from StackOverflow (don't have source)
        let index_of_space = query_string.find(' ').unwrap_or(0);
        return &query_string[..index_of_space];
    }

    query_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        // Test with command only
        let actual = get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        let actual = get_command_from_query_string("tw @jsjoeio");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}
