/*!
`whoa-german-numbers` is a crate that parses and converts written german numbers like "dreitausend"

Notably, they are un-segmented by spaces or dashes.
*/

pub mod to_number;
pub use to_number::text_to_num;

pub mod to_text;
pub use to_text::num_to_text;
