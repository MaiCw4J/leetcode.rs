// https://leetcode.com/problems/smallest-range-i/

pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
    let (mut max, mut min) = (a[0], a[0]);
    for i in a {
        max = max.max(i);
        min = min.min(i);
    }
    0.max(max - min - 2 * k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![1];

        assert_eq!(0, smallest_range_i(input, 0));
    }

}