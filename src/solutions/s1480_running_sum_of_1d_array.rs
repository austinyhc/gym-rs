/**
 * [1480] Running Sum of 1d Array
 *
 * Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]&hellip;nums[i]).
 *
 * Return the running sum of nums.
 *
 *
 * <strong class="example">Example 1:
 *
 *
 * Input: nums = [1,2,3,4]
 * Output: [1,3,6,10]
 * Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
 *
 * <strong class="example">Example 2:
 *
 *
 * Input: nums = [1,1,1,1,1]
 * Output: [1,2,3,4,5]
 * Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].
 *
 * <strong class="example">Example 3:
 *
 *
 * Input: nums = [3,1,2,10,1]
 * Output: [3,4,6,16,17]
 *
 *
 *
 * Constraints:
 *
 *
 * 	1 <= nums.length <= 1000
 * 	-10^6 <= nums[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/running-sum-of-1d-array/
// discuss: https://leetcode.com/problems/running-sum-of-1d-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {

        let mut res: Vec<i32> = vec![];
        let mut sum: i32 = 0;

        for num in nums {
            sum += num;
            res.push(sum);
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1480() {
        assert_eq!(vec![1,3,6,10], Solution::running_sum(vec![1,2,3,4]));
        assert_eq!(vec![1,2,3,4,5], Solution::running_sum(vec![1,1,1,1,1]));
        assert_eq!(vec![3,4,6,16,17], Solution::running_sum(vec![3,1,2,10,1]));
    }
}

