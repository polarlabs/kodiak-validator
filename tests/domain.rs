use kodiak_validator::domain;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = domain::is_valid_domain("google.com");
        assert_eq!(result, true);
    }
}
