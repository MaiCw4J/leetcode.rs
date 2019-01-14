// https://leetcode.com/problems/plus-one/

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    for e in digits.iter_mut().rev() {
        // 这里不知道怎么写...
        if *e < 9 {
            *e += 1;
            return digits;
        }
        *e = 0;
    }
    // 到此处说明 输入为vec![9,9,9, ...]
    let mut vec = vec![0; digits.len() + 1]; // 定义一个长度为原长度+1的vec, 且元素均为0
    vec[0] = 1;
    return vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1,2,4], plus_one(vec![1,2,3]));
    }

}