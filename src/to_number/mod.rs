mod tokenize;
mod words;
use std::collections::HashSet;

fn calculate(tokens: Vec<String>) -> i32 {
    let is_multiple: HashSet<&str> =
        HashSet::from(["hundert", "tausend", "hunderttausend", "million"]);
    let mut sum = 0;
    let to_number = words::to_number();
    for w in tokens {
        let num = to_number.get(&w).unwrap_or(&0);
        if is_multiple.contains(&w.as_str()) {
            sum *= num;
            continue;
        }
        sum += num;
    }
    sum
}

/// Convert a german word to a integer
pub fn text_to_num(s: &str) -> i32 {
    let tokens = tokenize::to_tokens(s);
    let sum = calculate(tokens);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_num() {
        assert_eq!(15, text_to_num("fünfzehnte"));
        assert_eq!(15, text_to_num("fünfzehn"));
    }
}
