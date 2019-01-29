// https://leetcode.com/problems/monotonic-array/

pub fn is_monotonic(a: Vec<i32>) -> bool {
    let (mut inc, mut dec) = (true, true);
    for i in 1..a.len() {
        inc &= a[i - 1] <= a[i];
        dec &= a[i - 1] >= a[i];
    }
    inc || dec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 2, 2, 3];
        assert!(is_monotonic(input));

        let input_ii = vec![1, 2, 4, 3];
        assert!(is_monotonic(input_ii));
    }

}