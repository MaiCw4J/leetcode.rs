// https://leetcode.com/problems/robot-return-to-origin/

// UD，LR 抵消 即相对操作数量相等
pub fn judge_circle(moves: String) -> bool {
    let (mut h, mut v) = (0, 0);
    for e in moves.chars() {
        match e {
            'U' => h += 1,
            'D' => h -= 1,
            'L' => v += 1,
            'R' => v += 1,
            _ => {}
        }
    }
    h == 0 && v == 0
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, judge_circle("UUDD".to_owned()));
    }

}