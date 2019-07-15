// https://leetcode.com/problems/height-checker/

pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut copy = heights.clone();
    copy.sort_unstable();
    let mut count = 0;
    for i in 0..heights.len() {
        if let (Some(a), Some(b)) = (copy.get(i), heights.get(i)) {
            if a != b {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 1, 4, 2, 1, 3];
        let output = 3;
        assert_eq!(output, height_checker(input));
    }

}