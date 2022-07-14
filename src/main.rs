mod lib;
mod words;

fn main() {
    let s = lib::num_to_text(3);
    println!("\n{:?}\n", s);
    // data::generate();
    let words = words::generate();
    println!("\n{:?}\n", words);
    // let words = Vec::from([("foo", "bar", "baz"), ("foo", "bar", "baz")]);
    // println!("\n{:?}\n", words[0].0);
    println!("\n{:?}\n", words::SMALLS[0]);
}
