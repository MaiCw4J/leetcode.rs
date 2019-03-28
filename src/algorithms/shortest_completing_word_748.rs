// https://leetcode.com/problems/shortest-completing-word/

use std::collections::HashSet;

pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    "".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let license_plate = "1s3 PSt".to_owned();
        let words = vec!["step".to_owned(), "steps".to_owned(), "stripe".to_owned(), "stepple".to_owned()];
        let output = "steps".to_owned();
        assert_eq!(output, shortest_completing_word(license_plate, words));
    }

}