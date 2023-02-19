/**
 * [169] Majority Element
 *
 * Given an array nums of size n, return the majority element.
 * The majority element is the element that appears more than &lfloor;n / 2&rfloor; times. You may assume that the majority element always exists in the array.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [3,2,3]
 * Output: 3
 * <strong class="example">Example 2:
 * Input: nums = [2,2,1,1,1,2,2]
 * Output: 2
 *  
 * Constraints:
 * 
 * 	n == nums.length
 * 	1 <= n <= 5 * 10^4
 * 	-10^9 <= nums[i] <= 10^9
 * 
 *  
 * Follow-up: Could you solve the problem in linear time and in O(1) space?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/majority-element/
// discuss: https://leetcode.com/problems/majority-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Option<i32> {
        let total = nums.len() as u32;
        let mut dict: HashMap<i32,u32> = HashMap::new();
        let mut majority: Option<i32> = None;
        let mut max_ratio = f32::MIN;

        for num in nums {
            if let Some(&count) = dict.get(&num) {

                dict.insert(num, count+1);

                let ratio: f32 = ((count+1) / total) as f32;
                if ratio > max_ratio {
                    max_ratio = ratio;
                    majority = Some(num);
                }
            } else {
                dict.insert(num, 1);
            }
        }
        majority
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_169() {
        assert_eq!(Some(3), Solution::majority_element(vec![3,2,3]));
        assert_eq!(Some(2), Solution::majority_element(vec![2,2,1,1,1,2,2]));
        assert_eq!(Some(1), Solution::majority_element(vec![1,2,3,4,5,6,7,1]));
        assert_eq!(None, Solution::majority_element(vec![1,2,3,4,5,6,7,8]));
        assert_eq!(None, Solution::majority_element(vec![]));
    }
}

