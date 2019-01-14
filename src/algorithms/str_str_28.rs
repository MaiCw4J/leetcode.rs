// https://leetcode.com/problems/implement-strstr/

pub fn str_str(haystack: String, needle: String) -> i32 {
    let (haystack, needle) = (haystack.into_bytes(), needle.into_bytes());
    let (haystack_len, needle_len) = (haystack.len(), needle.len());
    let mut i= 0;
    loop {
        let mut j= 0;
        loop {
            if j == needle_len {
                return i as i32;
            }
            if i + j == haystack_len {
                return -1;
            }
            if haystack[i + j] != needle[j] {
                break;
            }
            j += 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(-1, str_str("aaaaa".to_owned(),"baa".to_owned()))
    }

}