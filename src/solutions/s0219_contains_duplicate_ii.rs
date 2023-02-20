/**
 * [219] Contains Duplicate II
 *
 * Given an integer array nums and an integer k, return true if there are two distinct indices i and j
 * in the array such that nums[i] == nums[j] and abs(i - j) <= k.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [1,2,3,1], k = 3
 * Output: true
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,0,1,1], k = 1
 * Output: true
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums = [1,2,3,1,2,3], k = 2
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	0 <= k <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/contains-duplicate-ii/
// discuss: https://leetcode.com/problems/contains-duplicate-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        use std::collections::hash_map::Entry;

        let mut dict = HashMap::<i32, bool>::new();

        for (i, &num) in nums.iter().enumerate() {

            if let Entry::Vacant(entry) = dict.entry(num) {
                entry.insert(true)
            } else { return true; };

            if dict.len() > k as usize {
                dict.remove(&nums[i - k as usize]);
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
    fn test_219() {
        assert_eq!(true, Solution::contains_nearby_duplicate(vec![1,2,3,1], 3));
        assert_eq!(true, Solution::contains_nearby_duplicate(vec![1,0,0,1], 1));
        assert_eq!(false, Solution::contains_nearby_duplicate(vec![1,2,3,1,2,3], 2));
    }
}

