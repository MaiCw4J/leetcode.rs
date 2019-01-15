// https://leetcode.com/problems/number-complement/

// 方法一
pub fn find_complement(num: i32) -> i32 {
    let (mut i, mut j) = (0, 0);
    while i < num {
        i += 2_i32.pow(j); // 2的j次方 （2必须显式标注为i32类型)
        j += 1;
    }
    i - num
}

// 方法二
pub fn find_complement_ii(num: i32) -> i32 {
    let mut mask = !0;
    while (num & mask) > 0 {
        mask <<= 1;
    }
    !mask & !num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, find_complement(5));

        assert_eq!(2, find_complement_ii(5));
    }

}