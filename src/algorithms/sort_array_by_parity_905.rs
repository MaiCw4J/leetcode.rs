// https://leetcode.com/problems/sort-array-by-parity/

pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
    let mut a = a;
    let mut j = 0;
    for i in 0..a.len() {
        if a[i] & 1 == 0 {
            a.swap(i, j);
            j += 1;
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![3,1,2,4];
        let output = vec![2,4,3,1];
        assert_eq!(output, sort_array_by_parity(input));
    }

}