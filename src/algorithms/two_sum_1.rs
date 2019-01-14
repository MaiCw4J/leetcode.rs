// https://leetcode.com/problems/two-sum/
use std::collections::HashMap;

pub fn two_sum(nums: &mut Vec<i32>, target: i32) -> Vec<usize> {
    let mut cache = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        match cache.get(n) {
            Some(v) => return vec![*v, i],
            None => cache.insert(target - n, i)
        };
    }
    Vec::new()
}


#[cfg(test)]
mod tests {
    use crate::algorithms::two_sum_1::two_sum;

    #[test]
    fn two_sum_test() {
        let mut nums = vec![2, 7, 11, 15];
        assert_eq!(vec![0, 1], two_sum(&mut nums, 9));

        assert_eq!(vec![0, 3], two_sum(&mut nums, 17));

        assert_eq!(vec![0, 2], two_sum(&mut nums, 13));
    }
}