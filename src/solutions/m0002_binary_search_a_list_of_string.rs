/**
 * [0001] Matt's Parking Grarage
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn binary_search(names: &Vec<&str>, target: String) -> i32 {
        let mut left: i32 = -1;
        let mut right: i32 = names.len() as i32;

        while left+1 != right {
            let mid = left + (right - left) / 2;
            let name = names[mid as usize].to_owned();
            
            if target <= name {
                right = mid;
            } else {
                left = mid;
            }
        }
        if names[right as usize] != target { return -1; } 
        right
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1480() {
        let names: Vec<&str> = vec![ "austin", "chi", "fran", "gemma", "ginny",
                                     "gino", "ken", "sam", "sean", "tracy"];

        assert_eq!(0, Solution::binary_search(&names, format!("austin")));
        assert_eq!(3, Solution::binary_search(&names, format!("gemma")));
        assert_eq!(-1, Solution::binary_search(&names, format!("amy")));
        assert_eq!(-1, Solution::binary_search(&names, format!("stan")));
    }
}

