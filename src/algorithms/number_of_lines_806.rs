// https://leetcode.com/problems/number-of-lines-to-write-string/

pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let (mut cur, mut res) = (0, 1);
    let a_ascii = 'a' as u8; // 'a' 字符 ASCII码
    for c in s.bytes() {
        let index = (c - a_ascii) as usize;
        let v = widths[index]; // 获取字符的位置
        if cur + v > 100 {
            res += 1; // 换行
            cur = v; // 新一行就使用当前字符的宽度
        } else {
            cur += v;
        }
    }
    vec![res, cur]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let widths = vec![10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10];
        let res = vec![3,60];
        assert_eq!(res, number_of_lines(widths, "abcdefghijklmnopqrstuvwxyz".to_ascii_lowercase()));
    }

}