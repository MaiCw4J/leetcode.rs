// https://leetcode.com/problems/sqrtx/

pub fn my_sqrt(x: i32) -> i32 {
    let mut l = 0;
    let mut r = x;
    while l < r {
        let c = (l + r + 1) / 2;
        if c > x / c {
            r = c - 1;
        } else {
            l = c;
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, my_sqrt(8))
    }

}