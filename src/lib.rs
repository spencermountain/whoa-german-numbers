/*!
`whoa-german-numbers` is a crate that parses and converts written german numbers like "dreitausend" back and forth into integers.


Notably, numbers are not segmented by spaces or dashes in german - meaning they can be quite unruly sometimes.

### Usage
```
use whoa_german_numbers::{number_to_text, text_to_number};

let num = text_to_number("zwölf");
assert_eq!(num.unwrap(), 12);

let str = number_to_text(121);
assert_eq!(str, "einhunderteinundzwanzig");
```
*/

/// Convert written german numbers like "dreitausend" to integers (like 3000).
/// # Example
/// ```
/// use whoa_german_numbers::text_to_number;
///
/// assert_eq!(text_to_number("fünf").unwrap(), 5);
/// assert_eq!(text_to_number("fünfte").unwrap(), 5);
/// assert_eq!(text_to_number("zweihundertzehn").unwrap(), 210);
/// ```
/// returns an Option<i32> which is None when no valid number is found
///
/// ```
/// use whoa_german_numbers::text_to_number;
///
/// assert_eq!(text_to_number("foobar").is_none(), true);
/// ```
pub mod to_number;
pub use to_number::to_number as text_to_number;

/// Convert an integer (like 81), into a german text number, like "einundachtzig".
/// # Example
/// ```
/// use whoa_german_numbers::number_to_text;
///
/// assert_eq!(number_to_text(81), "einundachtzig");
/// assert_eq!(number_to_text(5), "fünf");
/// ```
pub mod to_text;
pub use to_text::to_text as number_to_text;
