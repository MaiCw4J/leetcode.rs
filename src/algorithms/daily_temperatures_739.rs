// https://leetcode.com/problems/daily-temperatures/

// stack
pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    let mut res = vec![0; t.len()];
    for (i, e) in t.iter().enumerate() {
        while let Some(v) = stack.last().filter(|s| *e > t[**s]) {
            res[*v] = (i - *v) as i32;
            stack.pop();
        }
        stack.push(i);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let output = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(output, daily_temperatures(input));
    }

}