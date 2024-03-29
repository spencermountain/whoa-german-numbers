// pub const ARR: [i32; 4] = [3, 1, 4, 2];
use std::collections::HashMap;

pub const SMALLS: [(i32, &str, &str); 19] = [
    (1, "eins", "erste"),
    (2, "zwei", "zweite"),
    (3, "drei", "dritte"),
    (4, "vier", "vierte"),
    (5, "fünf", "fünfte"),
    (6, "sechs", "sechste"),
    (7, "sieben", "siebente"),
    (8, "acht", "achte"),
    (9, "neun", "neunte"),
    (10, "zehn", "zehnte"),
    (11, "elf", "elfte"),
    (12, "zwölf", "zwölfte"),
    (13, "dreizehn", "dreizehnte"),
    (14, "vierzehn", "vierzehnte"),
    (15, "fünfzehn", "fünfzehnte"),
    (16, "sechzehn", "sechzehnte"),
    (17, "siebzehn", "siebzehnte"),
    (18, "achtzehn", "achtzehnte"),
    (19, "neunzehn", "neunzehnte"),
];
pub const TENS: [(i32, &str, &str); 8] = [
    (20, "zwanzig", "zwanzigste"),
    (30, "dreißig", "dreißigste"),
    (40, "vierzig", "vierzigste"),
    (50, "fünfzig", "fünfzigste"),
    (60, "sechzig", "sechzigste"),
    (70, "siebzig", "siebzigste"),
    (80, "achtzig", "achtzigste"),
    (90, "neunzig", "neunzigste"),
];
pub const MULTIPLES: [(i32, &str, &str); 4] = [
    (100, "hundert", "hundertste"),
    (1000, "tausend", "tausendste"),
    (100000, "hunderttausend", "hunderttausendste"),
    (1000000, "million", "millionste"),
];

pub fn to_number() -> HashMap<String, i32> {
    let mut words = HashMap::new();
    for tup in SMALLS {
        words.insert(tup.1.to_string(), tup.0);
        words.insert(tup.2.to_string(), tup.0);
    }
    for tup in TENS {
        words.insert(tup.1.to_string(), tup.0);
        words.insert(tup.2.to_string(), tup.0);
    }
    for tup in MULTIPLES {
        words.insert(tup.1.to_string(), tup.0);
        words.insert(tup.2.to_string(), tup.0);
    }
    // add misc
    words.insert("ein".to_string(), 1);
    words
}

pub fn all_words() -> Vec<String> {
    let mut words = Vec::new();
    for tup in SMALLS {
        words.push(tup.1.to_string());
        words.push(tup.2.to_string());
    }
    for tup in TENS {
        words.push(tup.1.to_string());
        words.push(tup.2.to_string());
    }
    for tup in MULTIPLES {
        words.push(tup.1.to_string());
        words.push(tup.2.to_string());
    }
    // add misc
    words.push("und".to_string());
    words.push("ein".to_string());
    // sort words by longest-first
    words.sort_by(|a, b| b.len().cmp(&a.len()));
    words
}
