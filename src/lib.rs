pub fn twofer(name: &str) -> String {
    if name.is_empty() {
        "One for you, one for me.".to_string()
    } else {
        format!("One for {}, one for me.", name)
    }
}
