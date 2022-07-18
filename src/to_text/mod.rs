mod big;
mod words;

fn two_digit(num: i32) -> (i32, String) {
    let mut num = num;
    let mut out = String::new();
    // look for 'fifty'
    if num > 19 {
        for tup in words::TENS {
            if num >= tup.0 {
                // println!("\n{:?}\n", tup);
                out += tup.1;
                num -= tup.0
            }
        }
    }
    // look for 'three'
    for tup in words::SMALLS {
        if num >= tup.0 {
            let mut w = tup.1;
            if w == "eins" {
                w = "ein";
            }
            //ein und achtzig
            out = format!("{}und{}", w, out);
            num -= tup.0
        }
    }
    (num, out)
}

fn one_digit(num: i32) -> (i32, String) {
    let mut num = num;
    let mut out = String::new();
    // look for 'three'
    for tup in words::SMALLS {
        if num >= tup.0 {
            // println!("\n{:?}\n", tup);
            out += tup.1;
            num -= tup.0
        }
    }
    (num, out)
}

/// Convert an integer to its german word
pub fn num_to_text(n: i32) -> String {
    let mut out = String::new();
    let mut num = n;
    //fail-fast
    if n == 0 {
        return String::from("null");
    }
    // support negative
    if n < 0 {
        out += "moins ";
        num *= -1;
    }
    if num >= 100 {
        let res = big::big_numbers(num);
        num = res.0;
        out += &res.1;
    }
    if num > 19 {
        let res = two_digit(num);
        num = res.0;
        out += &res.1;
    }
    let res = one_digit(num);
    out += &res.1;
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_text() {
        assert_eq!(num_to_text(81), "einundachtzig".to_string());
    }
}
