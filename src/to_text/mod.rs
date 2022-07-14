/// Convert an integer to its german word
pub fn num_to_text(n: i32) -> String {
    println!("\n{:?}\n", n);
    // todo!("ready");
    "foo".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_text() {
        let result = num_to_text(22);
        assert_eq!(result, "foo".to_string());
    }
}
