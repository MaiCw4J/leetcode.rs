// https://leetcode.com/problems/stone-game/

pub fn stone_game(piles: Vec<i32>) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![5, 100, 4, 5];
        assert!(stone_game(input));
    }

}