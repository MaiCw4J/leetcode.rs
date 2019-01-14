// https://leetcode.com/problems/di-string-match/

pub fn di_string_match(s: String) -> Vec<i32> {
    let (mut l, mut r, len) = (0, s.len() as i32, s.len());
    let mut res: Vec<i32> = vec![0; len + 1];
    for (i, e) in s.chars().enumerate() {
        if e == 'I' {
            res[i] = l;
            l += 1;
        } else {
            res[i] = r;
            r -= 1;
        }
    }
    res[len] = l;
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 4, 1, 3, 2], di_string_match("IDID".to_owned()));
    }

}