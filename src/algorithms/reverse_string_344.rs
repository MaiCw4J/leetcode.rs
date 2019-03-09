// https://leetcode.com/problems/reverse-string/

pub fn reverse_string(s: String) -> String {
    if s.is_empty() {
        return s;
    }
    let (mut i, mut j) = (0 as usize, s.len() - 1);
    let mut bytes = s.into_bytes();
    while i < j {
        // 交换两个值
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
        assert_eq!("olleh".to_owned(), reverse_string("hello".to_owned()));
    }

}