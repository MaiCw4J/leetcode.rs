// https://leetcode.com/problems/uncommon-words-from-two-sentences/

use std::collections::HashMap;

pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
    let concat = format!("{} {}", a, b);
    let mut m = HashMap::new();
    for s in concat.split_whitespace() {
        *m.entry(s).or_insert(0) += 1;
    }
    let mut result = vec![];
    for (k, v) in m {
        if v == 1 {
            result.push(k.to_owned());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let out_put = vec!["sweet".to_owned(), "sour".to_owned()];
        let a = "this apple is sweet".to_owned();
        let b = "this apple is sour".to_owned();
        assert_eq!(out_put, uncommon_from_sentences(a, b));

    }

}