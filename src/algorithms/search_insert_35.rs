// https://leetcode.com/problems/search-insert-position/

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    for (i, e) in nums.iter().enumerate() {
        if *e >= target {
            return i as i32;
        }
    }
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, search_insert(vec![1,3,5,6], 2))
    }

}