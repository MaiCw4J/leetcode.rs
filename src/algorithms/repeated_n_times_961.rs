// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/

pub fn repeated_n_times(a: Vec<i32>) -> i32 {
    for i in 2..a.len() {
        if a[i] == a[i - 1] || a[i] == a[i - 2] {
            return a[i];
        }
    }
    a[a.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, repeated_n_times(vec![1,2,3,3]))
    }

}