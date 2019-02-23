// https://leetcode.com/problems/roman-to-integer/
use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    // ASCII码
    let mut m = HashMap::new();
    m.insert(73, 1); // I
    m.insert(86, 5); // V
    m.insert(88, 10); // X
    m.insert(76, 50); // L
    m.insert(67, 100); // C
    m.insert(68, 500); // D
    m.insert(77, 1000); //M
    let m = m;
    let mut sum = 0;
    let bytes = s.into_bytes();
    let len = bytes.len() - 1;
    for i in 0..len {
        if let Some(v) = m.get(&bytes[i]) {
            if let Some(v1) = m.get(&bytes[i + 1]) {
                if v < v1 {
                    sum -= v;
                } else {
                    sum += v;
                }
            }
        }
    }
    sum + m[&bytes[len]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1994, roman_to_int("MCMXCIV".to_owned()));
    }
}