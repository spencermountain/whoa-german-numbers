> **Note**
>  it's my first crate, plz go easy on me!

# whoa-german-numbers
parse textual german number formats in rust

[![Current Version](https://img.shields.io/crates/v/whoa-german-numbers.svg)](https://crates.io/crates/whoa-german-numbers)
[![Documentation](https://docs.rs/whoa-german-numbers/badge.svg)](https://docs.rs/whoa-german-numbers)
[![License: MIT](https://img.shields.io/crates/l/whoa-german-numbers.svg)](#license)


Notably, numbers are not segmented by spaces or dashes in german - meaning they can be quite unruly sometimes.

### Usage
```
extern crate whoa_german_numbers;
use whoa_german_numbers::{number_to_text, text_to_number};

let num = text_to_number("zwölf");
assert_eq!(num.unwrap(), 12);

let str = number_to_text(121);
assert_eq!(str, "einhunderteinundzwanzig");
```
*/


### text_to_number - "fünf" -> 5
Convert written german numbers like "dreitausend" to integers (like 3000).

```rust
use whoa-german-numbers::text_to_number;

assert_eq!(text_to_number("fünf").unwrap(), 5);
assert_eq!(text_to_number("fünfte").unwrap(), 5); //ordinal support
assert_eq!(text_to_number("zweihundertzehn").unwrap(), 210);
```


### number_to_text - 5 -> "fünf"
Convert an integer (like 81), into a german text number, like "einundachtzig".
```rust
use whoa-german-numbers::number_to_text;

assert_eq!(number_to_text(81), "einundachtzig");
assert_eq!(number_to_text(5), "fünf");
assert_eq!(number_to_text(131), "einhunderteinunddreißig");
```

MIT