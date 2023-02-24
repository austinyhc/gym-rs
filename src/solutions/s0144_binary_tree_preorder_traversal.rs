/**
 * [144] Binary Tree Preorder Traversal
 *
 * Given the root of a binary tree, return the preorder traversal of its nodes' values.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/inorder_1.jpg" style="width: 125px; height: 200px;" />
 * Input: root = [1,null,2,3]
 * Output: [1,2,3]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: root = []
 * Output: []
 * 
 * <strong class="example">Example 3:
 * 
 * Input: root = [1]
 * Output: [1]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 100].
 * 	-100 <= Node.val <= 100
 * 
 *  
 * Follow up: Recursive solution is trivial, could you do it iteratively?
 * 
 */
pub struct Solution {}
use crate::utils::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-preorder-traversal/
// discuss: https://leetcode.com/problems/binary-tree-preorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        Self::preorder_traverse(root.as_ref(), &mut result);
        result
    }
    pub fn preorder_traverse(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if node.is_none() { return; }
        result.push(node.unwrap().borrow().val);
        Self::preorder_traverse(node.unwrap().borrow().left.as_ref(), result);
        Self::preorder_traverse(node.unwrap().borrow().right.as_ref(), result);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_144() {
        assert_eq!(vec![1,2,3], Solution::preorder_traversal(tree!(1,null,2,3)));
        assert_eq!(Vec::<i32>::new(), Solution::preorder_traversal(tree!()));
        assert_eq!(vec![1,4,6,5,3], Solution::preorder_traversal(tree!(1,4,5,6,null,null,3)));
    }
}

