use super::words;

fn find_prefix(str: &str, list: &Vec<String>) -> Option<String> {
    for w in list {
        if str.starts_with(&*w) {
            return Some(String::from(&*w));
        }
    }
    None
}

pub fn to_tokens(str: &str) -> Vec<String> {
    let list = words::all_words();
    let mut tokens: Vec<String> = Vec::new();
    let mut tmp = str.clone();

    loop {
        let res = find_prefix(tmp, &list);
        match res {
            Some(pre) => {
                tmp = tmp.strip_prefix(&pre).unwrap();
                tokens.push(pre);
            }
            None => {
                break;
            }
        }
    }
    tokens
}
