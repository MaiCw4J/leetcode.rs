// https://leetcode.com/problems/complement-of-base-10-integer/

pub fn bitwise_complement(n: i32) -> i32 {
    let mut c = 1;
    while c < n {
        c = (c << 1) + 1;
    }
    c ^ n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, bitwise_complement(7));
    }

}