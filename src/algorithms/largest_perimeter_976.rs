// https://leetcode.com/problems/largest-perimeter-triangle/

pub fn largest_perimeter(a: Vec<i32>) -> i32 {
    let mut a = a;
    a.sort();
    let mut i = a.len() - 1;
    while i >= 2 {
        if a[i] < a[i - 1] + a[i - 2] {
            return a[i] + a[i - 1] + a[i - 2];
        }
        i -= 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![2,1,2];
        assert_eq!(5, largest_perimeter(input))
    }

}