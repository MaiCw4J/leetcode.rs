// https://leetcode.com/problems/delete-columns-to-make-sorted/

pub fn min_deletion_size(a: Vec<String>) -> i32 {
    let len = a[0].len();
    let c = a.into_iter().map(String::into_bytes).collect::<Vec<Vec<u8>>>();
    let mut res = 0;
    for i in 0..len {
        for x in 0..c.len() - 1 {
            if c[x][i] > c[x + 1][i] {
                res += 1;
                break;
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
        let input = vec!["cba".to_owned(),"daf".to_owned(),"ghi".to_owned()];
        assert_eq!(1, min_deletion_size(input));
    }
    
}