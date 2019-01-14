// https://leetcode.com/problems/flipping-an-image/

// 比较位置对称的值，因为需要水平翻转，所以对称位置值不同，不需要垂直翻转值， 只需要改变对位相等的值
// 0 -> 1, 1 -> 0 使用^异或操作
pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut a = a;
    let len = a.len();
    for v in a.iter_mut() {
        let mut i = 0;
        while i * 2 < len {
            // 对称位置
            let j = len - i - 1;
            if v[i] == v[j] {
                v[i] ^= 1;
                v[j] = v[i];
            }
            i += 1;
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]], flip_and_invert_image(vec![vec![1,1,0], vec![1,0,1], vec![0,0,0]]))
    }

}