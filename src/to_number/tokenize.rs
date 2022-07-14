use super::words;

pub fn to_tokens(str: &str) -> Vec<&str> {
    let list = words::generate();
    let tokens = Vec::new();
    for w in list {
        if str.starts_with(&w) {
            println!("\n{:?}\n", w);
            // tokens.push(w);
        }
    }
    tokens
}
