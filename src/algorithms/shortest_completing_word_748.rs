// https://leetcode.com/problems/shortest-completing-word/

pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103];

    let get_char_product = |plate: &String| -> i64 {
        let mut i: i64 = 1;
        let bytes = plate.clone().into_bytes();
        for b in bytes {
            if b >= b'a' {
                let index = b - b'a';
                if index <= 25 {
                    i *= i64::from(primes[index as usize]);
                }
            }
        }
        i
    };

    let license = get_char_product(&license_plate.to_lowercase());
    let mut shortest = "aaaaaaaaaaaaaaaaaaaa".to_string();
    for word in words {
        if word.len() < shortest.len() && get_char_product(&word) % license == 0 {
            shortest = word;
        }
    }
    shortest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let license_plate = "1s3 PSt".to_owned();
        let words = vec!["step".to_owned(), "steps".to_owned(), "stripe".to_owned(), "stepple".to_owned()];
        let output = "steps".to_owned();
        assert_eq!(output, shortest_completing_word(license_plate, words));
    }

}