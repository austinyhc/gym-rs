/**
 * [383] Ransom Note
 *
 * Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.
 * Each letter in magazine can only be used once in ransomNote.
 *  
 * <strong class="example">Example 1:
 * Input: ransomNote = "a", magazine = "b"
 * Output: false
 * <strong class="example">Example 2:
 * Input: ransomNote = "aa", magazine = "ab"
 * Output: false
 * <strong class="example">Example 3:
 * Input: ransomNote = "aa", magazine = "aab"
 * Output: true
 *  
 * Constraints:
 * 
 * 	1 <= ransomNote.length, magazine.length <= 10^5
 * 	ransomNote and magazine consist of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ransom-note/
// discuss: https://leetcode.com/problems/ransom-note/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut dict = [0i32; 26];
        let magazine_bytes = magazine.into_bytes();
        let ransom_bytes = ransom_note.into_bytes();

        for byte in magazine_bytes {
            let index = byte - b'a';
            dict[index as usize] += 1;
        }

        for byte in ransom_bytes {
            let index = byte - b'a';
            if dict[index as usize] <= 0 { return false; }
            dict[index as usize] -= 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_383() {
        assert_eq!(false, Solution::can_construct(format!("a"), format!("b")));
        assert_eq!(false, Solution::can_construct(format!("aa"), format!("ab")));
        assert_eq!(true,  Solution::can_construct(format!("aa"), format!("aab")));
    }
}

