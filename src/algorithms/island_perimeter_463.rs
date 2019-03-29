// https://leetcode.com/problems/island-perimeter/

pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let (mut islands, mut neighbours) = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 1 {
                islands += 1;
                if i != 0 && grid[i - 1][j] == 1 {
                    neighbours += 1;
                }
                if j != 0 && grid[i][j - 1] == 1 {
                    neighbours += 1;
                }
            }
        }
    }
    (islands << 2) - (neighbours << 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![vec![0,1,0,0], vec![1,1,1,0], vec![0,1,0,0], vec![1,1,0,0]];
        assert_eq!(16, island_perimeter(input));
    }

}