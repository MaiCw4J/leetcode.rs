// https://leetcode.com/problems/keyboard-row/

use std::collections::HashSet;

pub fn find_words(words: Vec<String>) -> Vec<String> {
    let line = ["qwertyuiop", "asdfghjkl", "zxcvbnm"]
        .iter().map(|s| s.chars().into_iter().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();
    let mut stack = vec![];
    for word in words {
        let chars  = word.to_lowercase().chars().into_iter().collect::<HashSet<char>>();
        for s in &line {
            // a 是 b 的子集, b 包含 a
            if chars.is_subset(s) {
                stack.push(word);
                break;
            }
        }
    }
    stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec!["Hello".to_owned(), "Alaska".to_owned(), "Dad".to_owned(), "Peace".to_owned()];
        let output = vec!["Alaska".to_owned(), "Dad".to_owned()];
        assert_eq!(output, find_words(input));
    }

}