// https://leetcode.com/problems/remove-element/


pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut index: usize = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[index] = nums[i];
            index += 1;
        }
    }
    index as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, remove_element(&mut vec![3,2,2,3], 3))
    }

}