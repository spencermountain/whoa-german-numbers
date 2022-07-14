// pub const ARR: [i32; 4] = [3, 1, 4, 2];

#[allow(unused_variables, dead_code)]
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
#[allow(unused_variables, dead_code)]
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
#[allow(unused_variables, dead_code)]
pub const MULTIPLES: [(i32, &str, &str); 4] = [
    (100, "hundert", "hundertste"),
    (1000, "tausend", "tausendste"),
    (100000, "hunderttausend", "hunderttausendste"),
    (1000000, "million", "millionste"),
];

pub fn generate() -> Vec<String> {
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
    // sort words by longest-first
    words.sort_by(|a, b| b.len().cmp(&a.len()));
    words
}
