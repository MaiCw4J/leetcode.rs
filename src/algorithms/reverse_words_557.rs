// https://leetcode.com/problems/reverse-words-in-a-string-iii/

pub fn reverse_words(s: String) -> String {
    let mut b = s.into_bytes();
    let len = b.len();
    let mut i = 0 ;
    while i < len {
        if b[i] != b' ' {
            let mut j = i;
            while j + 1 < len && b[j + 1] != b' ' {
                j += 1;
            }
            let c = j;
            while i < j {
                b.swap(i, j);
                j -= 1;
                i += 1;
            }
            i = c;
        }
        i += 1;
    }
    String::from_utf8(b).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Let's take LeetCode contest".to_owned();
        let output = "s'teL ekat edoCteeL tsetnoc".to_owned();
        assert_eq!(output, reverse_words(input));
    }

}