// https://leetcode.com/problems/goat-latin/

pub fn to_goat_latin(s: String) -> String {
    let vowel = vec!['a','A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];
    let mut sb = String::new();
    let mut index = 1;
    for word in s.split_whitespace() {
        if index != 1 {
            sb.push_str(" ");
        }
        // 'c'第一个字符
        if let Some(c) = word.chars().next() {
            if vowel.contains(&c) {
                sb.push_str(word);
            } else {
                sb.push_str(&word[1..]);
                sb.push(c);
            }
            sb.push_str("ma");
            for _i in 0..index {
                sb.push('a');
            }
        }
        index += 1;
    }
    sb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "I speak Goat Latin".to_owned();
        let output = "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_owned();
        assert_eq!(output, to_goat_latin(input));
    }

}