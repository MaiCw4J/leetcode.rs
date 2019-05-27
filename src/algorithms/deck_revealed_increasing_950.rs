// https://leetcode.com/problems/reveal-cards-in-increasing-order/

pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
    let len = deck.len();
    let mut deck = deck;
    deck.sort();
    let mut queue = ::std::collections::vec_deque::VecDeque::new();
    for i in 0..len {
        queue.push_back(i as i32);
    }
    let mut res: Vec<i32> = vec![0; len];
    for i in deck {
        if let Some(index) = queue.pop_front() {
            res[index as usize] = i;
            if let Some(next) = queue.pop_front() {
                queue.push_back(next);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![17, 13, 11, 2, 3, 5, 7];
        let output = vec![2, 13, 3, 11, 5, 17, 7];
        assert_eq!(output, deck_revealed_increasing(input));
    }
}