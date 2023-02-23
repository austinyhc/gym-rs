/**
 * [0001] Check if substring
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn check_if_substring_in_rotated(s1: &str, s2: &str) -> bool {
        let s1 = s1.to_string() + s1;
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        
        for i in 0..=(s1.len() - s2.len()) {
            if s1[i..].iter().zip(s2).all(|(&c1, &c2)| { return c1 == c2; }) {
                return true;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1480() {
        assert_eq!(true, Solution::check_if_substring_in_rotated("AABCD", "CDAA"));
        assert_eq!(true, Solution::check_if_substring_in_rotated("AABCD", "ABCD"));
        assert_eq!(false, Solution::check_if_substring_in_rotated("AABCD", "ACBD"));
    }
}

