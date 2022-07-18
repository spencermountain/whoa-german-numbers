/*!
`whoa-german-numbers` is a crate that parses and converts written german numbers like "dreitausend" back and forth into integers.


Notably, german numbers are not segmented by spaces or dashes.
*/

/// Convert written german numbers like "dreitausend" to integers (like 3000).
/// # Example
/// ```
/// use whoa_german_numbers::text_to_num;
///
/// assert_eq!(text_to_num("fünf"), 5);
/// assert_eq!(text_to_num("fünfte"), 5);
/// assert_eq!(text_to_num("zweihundertzehn"), 210);
/// ```
pub mod to_number;
pub use to_number::text_to_num;

/// Convert an integer (like 81), into a german text number, like "einundachtzig".
/// # Example
/// ```
/// use whoa_german_numbers::num_to_text;
///
/// assert_eq!(num_to_text(81), "einundachtzig");
/// assert_eq!(num_to_text(5), "fünf");
/// ```
pub mod to_text;
pub use to_text::num_to_text;
