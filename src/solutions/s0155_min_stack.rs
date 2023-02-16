/**
 * [155] Min Stack
 *
 * Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
 * Implement the MinStack class:
 * 
 * 	MinStack() initializes the stack object.
 * 	void push(int val) pushes the element val onto the stack.
 * 	void pop() removes the element on the top of the stack.
 * 	int top() gets the top element of the stack.
 * 	int getMin() retrieves the minimum element in the stack.
 * 
 * You must implement a solution with O(1) time complexity for each function.
 *  
 * <strong class="example">Example 1:
 * 
 * Input
 * ["MinStack","push","push","push","getMin","pop","top","getMin"]
 * [[],[-2],[0],[-3],[],[],[],[]]
 * Output
 * [null,null,null,null,-3,null,0,-2]
 * Explanation
 * MinStack minStack = new MinStack();
 * minStack.push(-2);
 * minStack.push(0);
 * minStack.push(-3);
 * minStack.getMin(); // return -3
 * minStack.pop();
 * minStack.top();    // return 0
 * minStack.getMin(); // return -2
 * 
 *  
 * Constraints:
 * 
 * 	-2^31 <= val <= 2^31 - 1
 * 	Methods pop, top and getMin operations will always be called on non-empty stacks.
 * 	At most 3 * 10^4 calls will be made to push, pop, top, and getMin.
 * 
 */

// problem: https://leetcode.com/problems/min-stack/
// discuss: https://leetcode.com/problems/min-stack/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Default, Debug)]
pub struct MinStack {
    data_stack: Vec<i32>,
    min_stack: Vec<i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl MinStack {

    fn new() -> Self {
        Self::default()
    }
    
    fn push(&mut self, val: i32) {

        self.data_stack.push(val);

        if let Some(last) = self.min_stack.last() {
            if val <= *last {
                self.min_stack.push(val);
            }
        } else { 
            self.min_stack.push(val);
        }
    }
    
    fn pop(&mut self) {
        let data_top = self.data_stack.pop();
        let min_top = self.min_stack.last();
        if data_top == min_top.copied() {
            self.min_stack.pop();
        }
    }
    
    fn top(&mut self) -> Option<i32> {
        let t = self.data_stack.last().copied();
        self.pop();
        return t;
    }
    
    fn get_min(&self) -> Option<i32> {
        self.min_stack.last().copied()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_155_case_0() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(Some(-3), min_stack.get_min());
        min_stack.pop();
        assert_eq!(Some(0), min_stack.top());
        assert_eq!(Some(-2), min_stack.get_min());
        assert_eq!(Some(-2), min_stack.top());
        assert_eq!(None, min_stack.get_min());
    }

    #[test]
    fn test_155_case_1() {
        let mut min_stack = MinStack::new();
        assert_eq!(None, min_stack.get_min());
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(Some(-3), min_stack.get_min());
        min_stack.push(7);
        min_stack.push(-9);
        min_stack.push(-9);
        assert_eq!(Some(-9), min_stack.get_min());
        min_stack.pop();
        assert_eq!(Some(-9), min_stack.get_min());
        min_stack.pop();
        assert_eq!(Some(-3), min_stack.get_min());
    }
}

