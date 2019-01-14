// https://leetcode.com/problems/maximum-subarray/

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let (mut cul_max, mut all_max) = (nums[0], nums[0]);
    for i in 1..nums.len() {
        let e = nums[i];
        cul_max = e.max(e + cul_max);
        all_max = all_max.max(cul_max);
    }
    all_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
    }

}