// https://leetcode.com/problems/valid-parentheses/


// 遇到左边将转成对应右边的符合, 按顺序push到stack
// 遇到右边符合时,从stack中pop值与值比较，相同则继续下一个比较，不同直接返回false
// 判断stack是否空(表达式只有左边的操作符时)
pub fn is_valid(s: String) -> bool {
    // 判断个数是否为奇数
    if s.len() & 1 == 1 {
        return false;
    }
    let mut stack = vec![];
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            _ => {
                if !*stack.pop().map(|s| s == c).get_or_insert(false) {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(is_valid("((())){}{}[[[]]]".to_owned()));

        assert!(is_valid("(((((((".to_owned()));
    }

}