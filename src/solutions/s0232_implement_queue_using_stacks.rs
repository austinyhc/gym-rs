/**
 * [232] Implement Queue using Stacks
 *
 * Implement a first in first out (FIFO) queue using only two stacks. The implemented queue should support all the functions of a normal queue (push, peek, pop, and empty).
 * Implement the MyQueue class:
 * 
 * 	void push(int x) Pushes element x to the back of the queue.
 * 	int pop() Removes the element from the front of the queue and returns it.
 * 	int peek() Returns the element at the front of the queue.
 * 	boolean empty() Returns true if the queue is empty, false otherwise.
 * 
 * Notes:
 * 
 * 	You must use only standard operations of a stack, which means only push to top, peek/pop from top, size, and is empty operations are valid.
 * 	Depending on your language, the stack may not be supported natively. You may simulate a stack using a list or deque (double-ended queue) as long as you use only a stack's standard operations.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input
 * ["MyQueue", "push", "push", "peek", "pop", "empty"]
 * [[], [1], [2], [], [], []]
 * Output
 * [null, null, null, 1, 1, false]
 * Explanation
 * MyQueue myQueue = new MyQueue();
 * myQueue.push(1); // queue is: [1]
 * myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
 * myQueue.peek(); // return 1
 * myQueue.pop(); // return 1, queue is [2]
 * myQueue.empty(); // return false
 * 
 *  
 * Constraints:
 * 
 * 	1 <= x <= 9
 * 	At most 100 calls will be made to push, pop, peek, and empty.
 * 	All the calls to pop and peek are valid.
 * 
 *  
 * Follow-up: Can you implement the queue such that each operation is <a href="https://en.wikipedia.org/wiki/Amortized_analysis" target="_blank">amortized</a> O(1) time complexity? In other words, performing n operations will take overall O(n) time even if one of those operations may take longer.
 * 
 */

// problem: https://leetcode.com/problems/implement-queue-using-stacks/
// discuss: https://leetcode.com/problems/implement-queue-using-stacks/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Default, Debug)]
pub struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self::default()
    }
    
    fn push(&mut self, x: i32) {
        self.in_stack.push(x);
    }
    
    fn pop(&mut self) -> Option<i32> {

        if self.empty() { return None; }

        if self.out_stack.is_empty() {
            self.dump()
        }
        self.out_stack.pop()
    }
    
    fn peek(&mut self) -> Option<i32> {
        if self.empty() { return None; }

        if self.out_stack.is_empty() {
            self.dump()
        }
        self.out_stack.last().cloned()
    }

    fn dump(&mut self) {
        while let Some(element) = self.in_stack.pop() {
            self.out_stack.push(element);
        }
    }
    
    fn empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_232() {
        let mut q = MyQueue::new();
        assert_eq!(true, q.empty());
        q.push(1);
        q.push(2);
        assert_eq!(Some(1), q.peek());
        assert_eq!(Some(1), q.pop());
        assert_eq!(false, q.empty());
    }
}

