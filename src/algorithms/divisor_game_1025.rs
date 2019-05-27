// https://leetcode.com/problems/divisor-game/

pub fn divisor_game(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(divisor_game(2));
    }

}