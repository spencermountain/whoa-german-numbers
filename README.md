> **Note**
>  it's my first crate, plz go easy on me!

# whoa-german-numbers
parse textual german number formats in rust

[![Build Status](https://travis-ci.org/samueltardieu/whoa-german-numbers.svg?branch=master)](https://travis-ci.org/samueltardieu/whoa-german-numbers)
[![Current Version](https://img.shields.io/crates/v/whoa-german-numbers.svg)](https://crates.io/crates/whoa-german-numbers)
[![Documentation](https://docs.rs/whoa-german-numbers/badge.svg)](https://docs.rs/whoa-german-numbers)
[![License: Apache-2.0/MIT](https://img.shields.io/crates/l/whoa-german-numbers.svg)](#license)


### text_to_number - "fünf" -> 5
Convert written german numbers like "dreitausend" to integers (like 3000).

```rust
use whoa_german_numbers::text_to_number;

assert_eq!(text_to_number("fünf"), 5);
assert_eq!(text_to_number("fünfte"), 5); //ordinal support
assert_eq!(text_to_number("zweihundertzehn"), 210);
```


### number_to_text - 5 -> "fünf"
Convert an integer (like 81), into a german text number, like "einundachtzig".
```rust
use whoa_german_numbers::number_to_text;

assert_eq!(number_to_text(81), "einundachtzig");
assert_eq!(number_to_text(5), "fünf");
assert_eq!(number_to_text(131), "einhunderteinunddreißig");
```

MIT