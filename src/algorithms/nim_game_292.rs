// https://leetcode.com/problems/nim-game/

pub fn can_win_nim(n: i32) -> bool {
    n % 4 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(can_win_nim(3));
    }

}