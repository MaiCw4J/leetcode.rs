// https://leetcode.com/problems/distribute-candies/

pub fn distribute_candies(candies: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut bit = vec![0; 200001];
    for candy in &candies {
        let candy = (candy + 100000) as usize;
        if bit[candy] == 0 {
            count += 1;
            bit[candy] = 1;
        }
    }
    count.min((candies.len() / 2) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        assert_eq!(2, distribute_candies(vec![100000,0,100000,0,100000,0,100000,0,100000,0,100000,0]));
    }
    
}