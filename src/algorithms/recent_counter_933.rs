// https://leetcode.com/problems/number-of-recent-calls/

struct RecentCounter {
    queue: ::std::collections::vec_deque::VecDeque<i32>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            queue: ::std::collections::vec_deque::VecDeque::new()
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {

        while let Some(f) = self.queue.front() {
            if *f < (t - 3000) {
                self.queue.pop_front();
            } else {
                break;
            }
        }

        self.queue.push_back(t);

        self.queue.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut rc = RecentCounter::new();
        let input = vec![1, 100, 3001, 3002];
        for e in input {
            println!("res: {}", rc.ping(e));
        }
    }

}