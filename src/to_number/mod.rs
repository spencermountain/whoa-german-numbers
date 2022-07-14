mod tokenize;
mod words;

/// Convert a german word to a integer
pub fn text_to_num(s: &str) -> i32 {
    let tokens = tokenize::to_tokens(s);
    // let list = words::generate();
    println!("\n{:?}\n", tokens);
    println!("\n{:?}\n", s);
    // println!("\n{:?}\n", words::SMALLS);
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
}
