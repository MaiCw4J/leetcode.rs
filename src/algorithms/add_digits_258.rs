// https://leetcode.com/problems/add-digits/submissions/

// https://en.wikipedia.org/wiki/Digital_root#Congruence_formula
pub fn add_digits(num: i32) -> i32 {
    1 + (num - 1) % 9
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, add_digits(38));
    }

}