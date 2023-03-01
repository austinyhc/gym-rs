/**
 * [405] Convert a Number to Hexadecimal
 *
 * Given an integer num, return a string representing its hexadecimal representation. For negative integers, <a href="https://en.wikipedia.org/wiki/Two%27s_complement" target="_blank">two&rsquo;s complement</a> method is used.
 * All the letters in the answer string should be lowercase characters, and there should not be any leading zeros in the answer except for the zero itself.
 * Note: You are not allowed to use any built-in library method to directly solve this problem.
 *  
 * <strong class="example">Example 1:
 * Input: num = 26
 * Output: "1a"
 * <strong class="example">Example 2:
 * Input: num = -1
 * Output: "ffffffff"
 *  
 * Constraints:
 * 
 * 	-2^31 <= num <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/convert-a-number-to-hexadecimal/
// discuss: https://leetcode.com/problems/convert-a-number-to-hexadecimal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn to_hex(num: i32) -> String {
        let num = num as u32;
        let mapper = [
            '0', '1', '2', '3', '4',
            '5', '6', '7', '8', '9',
            'a', 'b', 'c', 'd', 'e', 'f',
        ];
        
        if num == 0 { return "0".to_owned(); }
        Self::to_hexer(num, &mapper)
    }


    pub fn to_hexer(num: u32, mapper: &[char]) -> String {
        
        if num == 0 { return String::new(); }
        let rem = (num & 15) as u8;
        let hex = mapper[rem as usize];
        
        return Self::to_hexer(num >> 4, mapper) + &hex.to_string();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_405() {
        assert_eq!("1a", Solution::to_hex(26));
        assert_eq!("1b", Solution::to_hex(27));
        assert_eq!("0", Solution::to_hex(0));
        assert_eq!("ffffffff", Solution::to_hex(-1));
        assert_eq!("fffffffe", Solution::to_hex(-2));
    }
}

