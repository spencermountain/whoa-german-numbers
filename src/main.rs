// mod lib;
// mod words;
// extern crate whoa-german-numbers; // not needed since Rust edition 2018
// use crate as whoa;
// use mylib::test;
use whoa_german_numbers::text_to_num;

fn main() {
    // let s = lib::num_to_text(3);
    // println!("\n{:?}\n", s);

    // let s = text_to_num("einundzwanzig");
    // let s = text_to_num("eintausendeinhunderteins");
    let s = text_to_num("hunderteins");
    println!("\n{:?}\n", s);
    // data::generate();
    // let words = words::generate();
    // println!("\n{:?}\n", words);
    // let words = Vec::from([("foo", "bar", "baz"), ("foo", "bar", "baz")]);
    // println!("\n{:?}\n", words[0].0);
    // println!("\n{:?}\n", words::SMALLS[0]);
}
