// https://leetcode.com/problems/max-consecutive-ones/

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let (mut temp, mut res) = (0, 0);
    for num in nums.into_iter() {
       if num == 1 {
           temp += 1;
           res = temp.max(res);
       } else {
           temp = 0;
       }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 1, 0, 1, 1, 1];
        assert_eq!(3, find_max_consecutive_ones(input));
    }

}