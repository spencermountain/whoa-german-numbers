mod tokenize;
mod words;
use std::collections::HashSet;

// derive a number from our tokens
fn calculate(tokens: Vec<String>) -> i32 {
    let mut sum = 0;
    let mut carry = 0;
    let mut minus = false;

    let is_multiple: HashSet<&str> =
        HashSet::from(["hundert", "tausend", "hunderttausend", "million"]);
    let to_number = words::to_number();

    for w in tokens {
        if w == "minus" {
            minus = true;
            continue;
        }
        //get num from word dictionary
        let num = to_number.get(&w).unwrap_or(&0);

        // support 'neun * hundert'
        if is_multiple.contains(&w.as_str()) {
            if carry == 0 {
                carry = 1;
            }
            // multiply anything infront of it
            sum += num * carry;
            carry = 0;
            continue;
        }
        carry += num;
    }

    // include any remaining
    if carry != 0 {
        sum += carry
    }
    // make it all negative
    if minus == true {
        sum *= -1
    }
    sum
}

/// Convert a german word to a integer
pub fn to_number(s: &str) -> Option<i32> {
    //this is the only way a 0 result is valid
    // 'null' is the german word for 0 ... :o
    if s == "null" {
        return Some(0);
    }
    // split "einhundert" to [ein, hundert]
    let tokens = tokenize::to_tokens(s);
    // produce a number
    let sum = calculate(tokens);
    // we found nothing, and not a valid 0
    if sum == 0 {
        return None;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_num() {
        assert_eq!(15, to_number("fünfzehnte").unwrap());
        assert_eq!(15, to_number("fünfzehn").unwrap());
    }
}
