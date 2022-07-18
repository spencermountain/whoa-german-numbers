use whoa_german_numbers::{number_to_text, text_to_number};

fn main() {
    let str = number_to_text(121);
    assert_eq!(str, "einhunderteinundzwanzig");

    let num = text_to_number("zwölf");
    assert_eq!(num.unwrap(), 12);
}
