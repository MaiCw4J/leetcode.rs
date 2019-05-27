// https://leetcode.com/problems/available-captures-for-rook/

pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    let cap = |board: &Vec<Vec<char>>,
               mut x: usize,
               mut y: usize,
               dx: i32,
               dy: i32| -> i32 {
        let mut x = x as i32;
        let mut y = y as i32;
        while x >= 0 &&
            x < board.len() as i32 &&
            y >= 0 && y < board[x as usize].len() as i32 &&
            board[x as usize][y as usize] != 'B' {
            if board[x as usize][y as usize] == 'p' {
                return 1;
            }
            x += dx;
            y += dy;
        }
        0
    };
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 'R' {
                return cap(&board, i, j, 0, 1) +
                cap(&board, i, j, 0, -1) +
                cap(&board, i, j, 1, 0) +
                cap(&board, i, j, -1, 0);
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
        let input = vec![vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                         vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                         vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
                         vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                         vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                         vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                         vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                         vec!['.', '.', '.', '.', '.', '.', '.', '.']];

        assert_eq!(3, num_rook_captures(input));
    }
}