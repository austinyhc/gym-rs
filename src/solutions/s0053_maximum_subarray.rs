/**
 * [53] Maximum Subarray
 *
 * Given an integer array nums, find the <span data-keyword="subarray-nonempty">subarray</span> with the largest sum, and return its sum.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
 * Output: 6
 * Explanation: The subarray [4,-1,2,1] has the largest sum 6.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1]
 * Output: 1
 * Explanation: The subarray [1] has the largest sum 1.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums = [5,4,-1,7,8]
 * Output: 23
 * Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 * 
 *  
 * Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-subarray/
// discuss: https://leetcode.com/problems/maximum-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut curr_sum = 0i32;

        for num in nums {
            curr_sum += num;
            curr_sum = if curr_sum > num { curr_sum } else { num };
            if curr_sum > max_sum { max_sum = curr_sum; }
        }
        max_sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53() {
        assert_eq!(-1, Solution::max_sub_array(vec![-1]));
        assert_eq!(-1, Solution::max_sub_array(vec![-1,-2,-3]));
        assert_eq!(-1, Solution::max_sub_array(vec![-3,-2,-1]));
        assert_eq!(6, Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
        assert_eq!(1, Solution::max_sub_array(vec![1]));
        assert_eq!(23, Solution::max_sub_array(vec![5,4,-1,7,8]));
    }
}

