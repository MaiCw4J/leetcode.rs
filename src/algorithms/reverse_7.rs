// https://leetcode.com/problems/reverse-integer/
pub fn reverse(x: i32) -> i32 {
    let (mut x, mut result) = (x, 0);
    while x != 0 {
        let tail = x % 10; // 取最后一位数
        let new_result = result * 10 + tail;
        if (new_result - tail) / 10 != result {
            return 0;
        }
        result = new_result;
        x /= 10; // 去掉个位数
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_test() {
        assert_eq!(321, reverse(123));

        assert_eq!(654, reverse(456));
    }
}