// https://leetcode.com/problems/transpose-matrix/

pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if a.is_empty() {
        return vec![];
    }
    let inner_len = a[0].len();
    let mut res = vec![vec![0 as i32; a.len()]; inner_len];
    for i in 0..inner_len {
        for j in 0..a.len() {
            res[i][j] = a[j][i];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let output = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(output, transpose(input));
    }

}