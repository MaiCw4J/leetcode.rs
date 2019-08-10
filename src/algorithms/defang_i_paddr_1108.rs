// https://leetcode.com/problems/defanging-an-ip-address/

pub fn defang_i_paddr(address: String) -> String {
    let mut res = String::new();
    for x in address.chars() {
        match x {
            '.' => res.push_str("[.]"),
            _ => res.push(x)
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "1.1.1.1".to_owned();
        let output = "1[.]1[.]1[.]1".to_owned();

        assert_eq!(output, defang_i_paddr(input));
    }
}