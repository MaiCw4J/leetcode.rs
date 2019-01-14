// https://leetcode.com/problems/hamming-distance/

// 1. x ^ y 异或 0110 ^ 0001 -> 0111
// 2. 利用&操作统计结果， (((((0111 & 0110(减一) -> 0110) & 0101(减一)) -> 0100) & 0011(减一)) -> 0000)
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let (mut count, mut xor) = (0, x ^ y);
    while xor > 0 {
        xor &= xor - 1;
        count += 1;
    }
    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, hamming_distance(1,6));
    }

}