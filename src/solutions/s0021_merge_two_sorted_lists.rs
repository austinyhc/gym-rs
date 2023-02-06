/**
 * [21] Merge Two Sorted Lists
 *
 * You are given the heads of two sorted linked lists list1 and list2.
 * Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
 * Return the head of the merged linked list.
 *
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg" style="width: 662px; height: 302px;" />
 * Input: list1 = [1,2,4], list2 = [1,3,4]
 * Output: [1,1,2,3,4,4]
 *
 * <strong class="example">Example 2:
 *
 * Input: list1 = [], list2 = []
 * Output: []
 *
 * <strong class="example">Example 3:
 *
 * Input: list1 = [], list2 = [0]
 * Output: [0]
 *
 *
 * Constraints:
 *
 * 	The number of nodes in both lists is in the range [0, 50].
 * 	-100 <= Node.val <= 100
 * 	Both list1 and list2 are sorted in non-decreasing order.
 *
 */
pub struct Solution {}
use crate::utils::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/merge-two-sorted-lists/
// discuss: https://leetcode.com/problems/merge-two-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>,
                           list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (None, None)     => None,
            (Some(l1), Some(l2)) => match l1.val < l2.val {
                true => Some(Box::new(ListNode {
                    val: l1.val,
                    next: Self::merge_two_lists(l1.next, Some(l2)),
                })),
                false => Some(Box::new(ListNode {
                    val: l2.val,
                    next: Self::merge_two_lists(Some(l1), l2.next),

                })),
            },
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterative_21() {
        assert_eq!(linked!(1,1,2,3,4,4),
            Solution::merge_two_lists(linked!(1,2,4), linked!(1,3,4)));
        assert_eq!(linked!(),
            Solution::merge_two_lists(linked!(), linked!()));
        assert_eq!(linked!(0),
            Solution::merge_two_lists(linked!(), linked!(0)));
        assert_eq!(linked!(1,2,8,10),
            Solution::merge_two_lists(linked!(1,10), linked!(2,8)));
    }

    #[test]
    fn test_recursive_21() {
        assert_eq!(linked!(1,1,2,3,4,4),
            Solution::merge_two_lists(linked!(1,2,4), linked!(1,3,4)));
        assert_eq!(linked!(),
            Solution::merge_two_lists(linked!(), linked!()));
        assert_eq!(linked!(0),
            Solution::merge_two_lists(linked!(), linked!(0)));
        assert_eq!(linked!(1,2,8,10),
            Solution::merge_two_lists(linked!(1,10), linked!(2,8)));
    }
}

