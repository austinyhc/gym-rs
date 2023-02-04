/**
 * [724] Find Pivot Index
 *
 * Given an array of integers nums, calculate the pivot index of this array.
 * The pivot index is the index where the sum of all the numbers strictly to the left of the index is equal to the sum of all the numbers strictly to the index's right.
 * If the index is on the left edge of the array, then the left sum is 0 because there are no elements to the left. This also applies to the right edge of the array.
 * Return the leftmost pivot index. If no such index exists, return -1.
 *
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,7,3,6,5,6]
 * Output: 3
 * Explanation:
 * The pivot index is 3.
 * Left sum = nums[0] + nums[1] + nums[2] = 1 + 7 + 3 = 11
 * Right sum = nums[4] + nums[5] = 5 + 6 = 11
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,3]
 * Output: -1
 * Explanation:
 * There is no index that satisfies the conditions in the problem statement.
 * <strong class="example">Example 3:
 *
 * Input: nums = [2,1,-1]
 * Output: 0
 * Explanation:
 * The pivot index is 0.
 * Left sum = 0 (no elements to the left of index 0)
 * Right sum = nums[1] + nums[2] = 1 + -1 = 0
 *
 *
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-1000 <= nums[i] <= 1000
 *
 *
 * Note: This question is the same as 1991: <a href="https://leetcode.com/problems/find-the-middle-index-in-array/" target="_blank">https://leetcode.com/problems/find-the-middle-index-in-array/</a>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-pivot-index/
// discuss: https://leetcode.com/problems/find-pivot-index/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut rsum: i32 = nums.iter().sum();
        let mut lsum: i32 = 0;

        for (i,num) in nums.iter().enumerate() {
            rsum -= num;
            if lsum == rsum { return i as i32; }
            lsum += num;
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_724() {
        assert_eq!(3,  Solution::pivot_index(vec![1,7,3,6,5,6]));
        assert_eq!(-1, Solution::pivot_index(vec![1,2,3]));
        assert_eq!(0,  Solution::pivot_index(vec![2,1,-1]));
    }
}

