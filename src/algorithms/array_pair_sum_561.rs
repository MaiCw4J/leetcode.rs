// https://leetcode.com/problems/array-partition-i/
// 排序后 取每组的第一个值，累计即为本题的解
pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    nums.into_iter().step_by(2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 4, 3, 2];
        assert_eq!(4, array_pair_sum(input));
    }

}