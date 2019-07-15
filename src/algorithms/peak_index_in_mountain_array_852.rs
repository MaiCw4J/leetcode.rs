// https://leetcode.com/problems/peak-index-in-a-mountain-array/

pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
    for i in 1..a.len() {
        if let (Some(a), Some(b)) = (a.get(i), a.get(i + 1)) {
            if a > b {
                return i as i32;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![0, 2, 1, 0];
        let output = 1;
        assert_eq!(output, peak_index_in_mountain_array(input));
    }

}