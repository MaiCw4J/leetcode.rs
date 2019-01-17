// https://leetcode.com/problems/fizz-buzz/

pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut result = vec!["".to_owned(); n as usize];
    for i in 1..=n {
        if let Some(elm) = result.get_mut((i - 1) as usize) {
            if i % 3 == 0 {
                elm.push_str("Fizz");
            }
            if i % 5 == 0 {
                elm.push_str("Buzz");
            }
            if elm.is_empty() {
                elm.push_str(&i.to_string())
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let out_put = vec![
            "1",
            "2",
            "Fizz",
            "4",
            "Buzz",
            "Fizz",
            "7",
            "8",
            "Fizz",
            "Buzz",
            "11",
            "Fizz",
            "13",
            "14",
            "FizzBuzz"
        ].iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(out_put, fizz_buzz(15));
    }
}