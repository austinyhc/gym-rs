/**
 * [621] Task Scheduler
 *
 * Given a characters array tasks, representing the tasks a CPU needs to do, where each letter represents a different task. Tasks could be done in any order. Each task is done in one unit of time. For each unit of time, the CPU could complete either one task or just be idle.
 * However, there is a non-negative integer n that represents the cooldown period between two same tasks (the same letter in the array), that is that there must be at least n units of time between any two same tasks.
 * Return the least number of units of times that the CPU will take to finish all the given tasks.
 *
 * <strong class="example">Example 1:
 *
 * Input: tasks = ["A","A","A","B","B","B"], n = 2
 * Output: 8
 * Explanation:
 * A -> B -> idle -> A -> B -> idle -> A -> B
 * There is at least 2 units of time between any two same tasks.
 *
 * <strong class="example">Example 2:
 *
 * Input: tasks = ["A","A","A","B","B","B"], n = 0
 * Output: 6
 * Explanation: On this case any permutation of size 6 would work since n = 0.
 * ["A","A","A","B","B","B"]
 * ["A","B","A","B","A","B"]
 * ["B","B","B","A","A","A"]
 * ...
 * And so on.
 *
 * <strong class="example">Example 3:
 *
 * Input: tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
 * Output: 16
 * Explanation:
 * One possible solution is
 * A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> idle -> idle -> A -> idle -> idle -> A
 *
 *
 * Constraints:
 *
 * 	1 <= task.length <= 10^4
 * 	tasks[i] is upper-case English letter.
 * 	The integer n is in the range [0, 100].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/task-scheduler/
// discuss: https://leetcode.com/problems/task-scheduler/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {

        let mut freqs = vec![0; 26];

        for &c in &tasks {
            let pos = c as usize - b'A' as usize;
            freqs[pos] += 1;
        }

        freqs.sort();

        let freq_max = freqs.pop().unwrap();
        let mut required_slots = (freq_max-1) * n;

        for freq in freqs {
             if freq == 0 { continue; }
             required_slots -= std::cmp::min(freq_max-1, freq);
             if required_slots < 0 {
                 required_slots = 0;
                 break;
             }
        }
        tasks.len() as i32 + required_slots
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_621() {
        assert_eq!(8, Solution::least_interval(vec!['A','A','A','B','B','B'], 2));
        assert_eq!(6, Solution::least_interval(vec!['A','A','A','B','B','B'], 0));
        assert_eq!(16, Solution::least_interval(
                vec!['A','A','A','A','A','A','B','C','D','E','F','G'], 2));

        assert_eq!(12, Solution::least_interval(
                vec!['A','A','A','B','B','B','C','C','C','D','D','E'], 2));
    }
}

