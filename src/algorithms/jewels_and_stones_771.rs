// https://leetcode.com/problems/jewels-and-stones/

use std::collections::HashSet;

pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let jewels_set: HashSet<char> = j.chars().into_iter().collect();
    s.chars().filter(|s| jewels_set.contains(s)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(123, num_jewels_in_stones("z".to_owned(),"ZZ".to_owned()));
    }

}