// https://leetcode.com/problems/reverse-only-letters/

pub fn reverse_only_letters(s: String) -> String {
    if s.is_empty() {
        return s;
    }
    let (mut i, mut j) = (0, s.len() - 1);
    let mut bytes = s.into_bytes();
    while i < j {
        while i < j && !bytes[i].is_ascii_alphabetic() {
            i += 1;
        }
        while i < j && !bytes[j].is_ascii_alphabetic() {
            j -= 1;
        }
        bytes.swap(i, j);
        i += 1;
        j -= 1;
    }
    String::from_utf8(bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "7_28]".to_owned();
        let output = "j-Ih-gfE-dCba".to_owned();
        assert_eq!(output, reverse_only_letters(input));
    }

}