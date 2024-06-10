pub fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
