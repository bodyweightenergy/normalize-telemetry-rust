pub fn say_hello() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use crate::hello::say_hello;

    #[test]
    fn test_hello() {
        assert_eq!(say_hello(), "Hello, world!");
    }
}
