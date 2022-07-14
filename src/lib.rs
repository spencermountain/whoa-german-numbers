/// Convert an integer to its german word
pub fn num_to_text(n: i32) -> String {
    println!("\n{:?}\n", n);
    // todo!("ready");
    "foo".to_string()
}

/// Convert a german word to a integer
pub fn text_to_num(s: &str) -> i32 {
    println!("\n{:?}\n", s);
    22
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_num() {
        let result = text_to_num("foo");
        assert_eq!(result, 22);
    }
    #[test]
    fn to_text() {
        let result = num_to_text(22);
        assert_eq!(result, "foo".to_string());
    }
}
