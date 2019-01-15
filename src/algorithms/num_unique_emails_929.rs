// https://leetcode.com/problems/unique-email-addresses/

use std::collections::HashSet;

pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    emails.iter().map(|s| {
        let split: Vec<&str> = s.split('@').collect();
        let first = split[0].split('+').collect::<Vec<&str>>()[0].replace(".", "");
        format!("{}@{}", first, split[1])
    }).collect::<HashSet<String>>().len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, num_unique_emails(vec!["test.email+alex@leetcode.com".to_owned(),"test.e.mail+bob.cathy@leetcode.com".to_owned(),"testemail+david@lee.tcode.com".to_owned()]))
    }

}