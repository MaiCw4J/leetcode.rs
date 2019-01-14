// https://leetcode.com/problems/palindrome-number/

pub fn is_palindrome(x: i32) -> bool {
    // 负数， 以及最后一位为0必为false
    if x < 0 || (x != 0 && x % 10 == 0){
        return false;
    }
    let mut x = x; // 重新绑定mut
    let mut cache = 0;
    while x > cache {
        cache = cache * 10 + x % 10;
        x /= 10;
    }
    x == cache || x == cache / 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(is_palindrome(121));
    }
}