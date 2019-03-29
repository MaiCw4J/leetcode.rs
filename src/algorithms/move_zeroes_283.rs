// https://leetcode.com/problems/move-zeroes/

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut j = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, j);
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut input = vec![0,1,0,3,12];
        move_zeroes(&mut input);
        assert_eq!(vec![1,3,12,0,0], input);
    }

}