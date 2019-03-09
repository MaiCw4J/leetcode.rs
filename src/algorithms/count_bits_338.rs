// https://leetcode.com/problems/counting-bits/

pub fn count_bits(num: i32) -> Vec<i32> {
    let num = num as usize;
    let mut r = vec![0; num + 1];
    for i in 1..=num {
        // 2的N次方
        r[i] = if i & !i == i {
            1
        } else {
            r[i >> 1] + (i & 1) as i32
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = 5;
        let output = vec![0,1,1,2,1,2];
        assert_eq!(output, count_bits(input));
    }

}