/**
 * [876] Middle of the Linked List
 *
 * Given the head of a singly linked list, return the middle node of the linked list.
 * If there are two middle nodes, return the second middle node.
 *
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-midlist1.jpg" style="width: 544px; height: 65px;" />
 * Input: head = [1,2,3,4,5]
 * Output: [3,4,5]
 * Explanation: The middle node of the list is node 3.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-midlist2.jpg" style="width: 664px; height: 65px;" />
 * Input: head = [1,2,3,4,5,6]
 * Output: [4,5,6]
 * Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.
 *
 *
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [1, 100].
 * 	1 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::utils::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/middle-of-the-linked-list/
// discuss: https://leetcode.com/problems/middle-of-the-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.as_ref();
        let mut slow = head.as_ref();

        while let Some(node) = fast {
            match node.next.as_ref() {
                None => break,
                Some(fast_next) => {
                    fast = fast_next.next.as_ref();
                    slow = slow.unwrap().next.as_ref();
                }
            }
        }
        slow.cloned()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_876() {
		assert_eq!(linked!(3,4,5), Solution::middle_node(linked!(1,2,3,4,5)));
		assert_eq!(linked!(4,5,6), Solution::middle_node(linked!(1,2,3,4,5,6)));
    }
}

