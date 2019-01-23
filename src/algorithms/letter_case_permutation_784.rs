// https://leetcode.com/problems/letter-case-permutation/

pub fn letter_case_permutation(s: String) -> Vec<String> {
    let mut res = vec![s.clone()];
    let len = s.len();
    let bytes = s.into_bytes();
    for i in 0..len {
        let (mut j, ll) = (0, res.len());
        while bytes[i].is_ascii_alphabetic() && j < ll {
            let mut a = res[j].clone().into_bytes();
            a[i] ^= 1 << 5;
            res.push(String::from_utf8(a).unwrap());
            j += 1;
        }
    }
    res
}

// 递归
pub fn letter_case_permutation_ii(s: String) -> Vec<String> {
    let mut s = s;
    let mut res = vec![];
    handle(s.as_mut_str(), 0, &mut res);
    res
}

fn handle(s: &mut str, i: usize, res: &mut Vec<String>) {
    if s.len() == i {
        res.push(s.to_owned());
        return;
    }
    handle(s, i + 1, res);
    let bytes = unsafe { s.as_bytes_mut() };
    if bytes[i].is_ascii_alphabetic()  {
        bytes[i] ^= 1 << 5;
        handle(s, i + 1, res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let output = vec!["a1b2".to_owned(), "a1B2".to_owned(), "A1b2".to_owned(), "A1B2".to_owned()];
        assert_eq!(output, letter_case_permutation("a1b2".to_owned()));

        assert_eq!(output, letter_case_permutation_ii("a1b2".to_owned()));
    }

}