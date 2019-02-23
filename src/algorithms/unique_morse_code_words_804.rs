// https://leetcode.com/problems/unique-morse-code-words/

use std::collections::HashSet;

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let d = vec![".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."];
    let a_ascii = b'a';
    let mut set = HashSet::new();
    for e in words {
        let res = e.into_bytes().iter()
            .map(|s| (s - a_ascii) as usize)
            .map(|s| d[s]).collect::<Vec<&str>>()
            .join("");
        set.insert(res);
    }
    set.len() as i32
}

// FP
pub fn unique_morse_representations_ii(words: Vec<String>) -> i32 {
    let d = vec![".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."];
    let a_ascii = b'a';
    words.into_iter()
         .map(|e| e.into_bytes().iter()
                   .map(|s| s - a_ascii)
                   .map(|s| s as usize)
                   .map(|s| d[s]).collect::<Vec<&str>>()
                   .join("")) // vec -> string
         .collect::<HashSet<String>>().len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec!["gin".to_owned(), "zen".to_owned(), "gig".to_owned(), "msg".to_owned()];
        assert_eq!(2, unique_morse_representations(words.clone()));
        assert_eq!(2, unique_morse_representations_ii(words));
    }
}