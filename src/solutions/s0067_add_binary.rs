/**
 * [67] Add Binary
 *
 * Given two binary strings a and b, return their sum as a binary string.
 *  
 * <strong class="example">Example 1:
 * Input: a = "11", b = "1"
 * Output: "100"
 * <strong class="example">Example 2:
 * Input: a = "1010", b = "1011"
 * Output: "10101"
 *  
 * Constraints:
 * 
 * 	1 <= a.length, b.length <= 10^4
 * 	a and b consist only of '0' or '1' characters.
 * 	Each string does not contain leading zeros except for the zero itself.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/add-binary/
// discuss: https://leetcode.com/problems/add-binary/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a: Vec<u8> = a.into_bytes().iter().map(|e| e-b'0').collect();
        let b: Vec<u8> = b.into_bytes().iter().map(|e| e-b'0').collect();
        let mut c: Vec<u8> = vec![];

        let mut i = (a.len() - 1) as i32;
        let mut j = (b.len() - 1) as i32;
        let mut carry = 0u8;

        while carry == 1 || j >= 0 || i >= 0 {
            let a_bit = if i < 0 { 0u8 } else { a[i as usize] };
            let b_bit = if j < 0 { 0u8 } else { b[j as usize] };
            let c_bit = a_bit ^ b_bit ^ carry;
            
            c.push(c_bit);

            carry = (a_bit + b_bit + carry) / 2;

            i -= 1;
            j -= 1;
        }
        let mut c: Vec<u8> = c.iter().map(|e| e+b'0').collect();
        c.reverse();
        String::from_utf8(c).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_67() {
        assert_eq!(format!("100"), Solution::add_binary(format!("11"), format!("1")));
        assert_eq!(format!("10101"), Solution::add_binary(format!("1010"), format!("1011")));
        assert_eq!(format!("10"), Solution::add_binary(format!("1"), format!("1")));
    }
}

