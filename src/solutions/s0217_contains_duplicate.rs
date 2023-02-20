/**
 * [217] Contains Duplicate
 *
 * Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [1,2,3,1]
 * Output: true
 * <strong class="example">Example 2:
 * Input: nums = [1,2,3,4]
 * Output: false
 * <strong class="example">Example 3:
 * Input: nums = [1,1,1,3,3,4,3,2,4,2]
 * Output: true
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/contains-duplicate/
// discuss: https://leetcode.com/problems/contains-duplicate/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        let mut dict = HashMap::<i32, bool>::new();

        for num in nums {
            if let Entry::Vacant(entry) = dict.entry(num) {
                entry.insert(true)
            } else { return true; };
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_217() {
        assert_eq!(true, Solution::contains_duplicate(vec![1,2,3,1]));
        assert_eq!(false, Solution::contains_duplicate(vec![1,2,3,4]));
        assert_eq!(true, Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]));
    }
}

