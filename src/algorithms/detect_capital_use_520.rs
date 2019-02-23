// https://leetcode.com/problems/detect-capital/

pub fn detect_capital_use(word: String) -> bool {
    let z_ascii = b'Z';
    let count = word.bytes().filter(|s| z_ascii >= *s).count();
    (count == 0 || count == word.len()) || (count == 1 && z_ascii >= word.into_bytes()[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Usaaa".to_owned();
        assert!(detect_capital_use(input));
    }

}