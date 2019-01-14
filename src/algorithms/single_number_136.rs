// https://leetcode.com/problems/single-number/

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut r = 0;
    for e in nums {
        r ^= e; // 异或 消除两两相等的数
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, single_number(vec![4,1,2,2,1]));
    }

}