// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

// 双指针 判断 i 与 i + 1位 是否相等 相等时原地删除i位置上的值，不相等时指针前移一位， 最后返回nums.len()
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut index: usize = 0;
    while index < nums.len() - 1 {
        if nums[index] == nums[index + 1] {
            nums.remove(index);
        } else {
            index += 1;
        }
    }
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]))
    }

}