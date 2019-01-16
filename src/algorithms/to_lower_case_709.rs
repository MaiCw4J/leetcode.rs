// https://leetcode.com/problems/to-lower-case/

// ASCII码表 大小写字母相差32
pub fn to_lower_case(str: String) -> String {
    let chars = str.bytes().map(|mut c| {
        if c >= 65 && c <= 90 {
            c += 32;
        }
        c
    }).collect::<Vec<u8>>();
    String::from_utf8(chars).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("aazz".to_owned(), to_lower_case("AaZz".to_owned()))
    }

}