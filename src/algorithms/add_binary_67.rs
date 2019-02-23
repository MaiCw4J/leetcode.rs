// https://leetcode.com/problems/add-binary/

pub fn add_binary(a: String, b: String) -> String {
    let mut res = String::new();
    let (a, b) = (a.into_bytes(), b.into_bytes());
    let (mut index_a, mut index_b) = ((a.len() - 1) as i32, (b.len() - 1) as i32);
    let (mut cache, zero_char) = (0, 48);
    while index_a >= 0 || index_b >= 0 || cache == 1 {
        let mut sum = cache;
        if index_a >= 0 {
            sum += a[index_a as usize] - zero_char; // '0' ASCII
            index_a -= 1;
        }
        if index_b >= 0 {
            sum += b[index_b as usize] - zero_char; // '0' ASCII
            index_b -= 1;
        }
        res.push_str(&format!("{}", sum % 2));
        cache = sum / 2;
    }
    if cache == 1 {
        res.push_str("1");
    }
    res.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("1000".to_owned(), add_binary("1".to_owned(), "111".to_owned()))
    }

}