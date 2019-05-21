// https://leetcode.com/problems/remove-outermost-parentheses/

pub fn remove_outer_parentheses(s: String) -> String {
    let mut res = String::from("");
    let mut opened = 0;
    for c in s.chars() {
        match c {
            '(' => {
                if opened > 0 { res.push(c); }; opened += 1;
            },
            ')' => {
                if opened > 1 { res.push(c); }; opened -= 1;
            },
            _ => {}
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "(()())(())".to_owned();
        let output = "()()()".to_owned();
        assert_eq!(output, remove_outer_parentheses(input));
    }

}