// https://leetcode.com/problems/next-greater-element-i/

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut res = nums1;
    let mut stack = vec![];
    let mut m = HashMap::new();
    for v in nums2 {
        while stack.last().map_or(false, |s| *s < v) {
            if let Some(a) = stack.pop() {
                m.insert(a, v);
            }
        }
        stack.push(v);
    }
    for i in res.iter_mut() {
        *i = match m.get(i) {
            Some(v) => *v,
            None => -1
        };
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![-1, 3, -1], next_greater_element(vec![4,1,2], vec![1,3,4,2]));
    }

}