// https://leetcode.com/problems/fair-candy-swap/

pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let dif = (a.iter().sum::<i32>() - b.iter().sum::<i32>()) / 2;
    for e in b {
        let sum = e + dif;
        if a.contains(&sum) {
            return vec![sum, e];
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input_i = vec![1, 1];
        let input_ii = vec![2, 2];
        let output = vec![1, 2];
        assert_eq!(output, fair_candy_swap(input_i, input_ii));
    }

}