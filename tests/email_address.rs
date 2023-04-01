use kodiak_validator::email_address;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = email_address::add(2, 2);
        assert_eq!(result, 4);
    }
}
