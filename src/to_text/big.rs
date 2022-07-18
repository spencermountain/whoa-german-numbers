use super::words;

pub fn big_numbers(num: i32) -> (i32, String) {
    let mut num = num;
    let mut out = String::new();

    for m in words::MULTIPLES {
        let (mult, word) = m;
        if num >= mult {
            // look for 'three' * 'hundred'
            for tup in words::SMALLS {
                if num >= tup.0 * mult {
                    let mut w = tup.1;
                    if w == "eins" {
                        w = "ein";
                    }
                    out += w;
                    out += word;
                    num -= tup.0 * mult
                }
            }
        }
    }
    (num, out)
}
