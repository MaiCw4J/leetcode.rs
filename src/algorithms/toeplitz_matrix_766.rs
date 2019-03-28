// https://leetcode.com/problems/toeplitz-matrix/

pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    for i in 0..matrix.len() - 1 {
        for j in 0..matrix[i].len() - 1 {
            if matrix[i][j] != matrix[i + 1][j + 1] {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]];
        assert!(is_toeplitz_matrix(input));
    }
}