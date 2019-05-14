// https://leetcode.com/problems/squares-of-a-sorted-array/

pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    let len = a.len();
    // 双指针
    let (mut i, mut j) = (0, len - 1);

    let mut res = vec![0; len];
    for p in (0..len).rev() {
        let (ai, aj) = (a[i], a[j]);
        if ai.abs() > aj.abs() {
            res[p] = ai * ai;
            i += 1;
        } else {
            res[p] = aj * aj;
            j -= 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![-4,-1,0,3,10];
        let output = vec![0,1,9,16,100];
        assert_eq!(output, sorted_squares(input));
    }

}