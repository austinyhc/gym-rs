/**
 * [0001] Matt's Parking Grarage
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn parking_status(tickets: Vec<(i32,i32)>, n: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut entries = HashMap::<i32, i32>::new();
        let mut exits   = HashMap::<i32, i32>::new();
        
        for ticket in tickets {
            entries.entry(ticket.0).and_modify(|c| *c += 1 ).or_insert(1);
            exits.entry(ticket.1 + 1).and_modify(|c| *c += 1 ).or_insert(1);
        }
        
        let mut result = vec![0i32; n as usize];

        for nday in 0..n {

            if nday > 0 { result[nday as usize] = result[(nday-1) as usize]; }
            if let Some(&cnt) = entries.get(&nday) { result[nday as usize] += cnt; }
            if let Some(&cnt) = exits.get(&nday)   { result[nday as usize] -= cnt; }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1480() {
        assert_eq!(vec![0,1,2,2,1],
            Solution::parking_status(vec![(1,3),(2,4)], 5));
        assert_eq!(vec![0,2,3,3,2,1,1,1,1,1],
            Solution::parking_status(vec![(1,3),(1,4),(2,9)], 10));
        assert_eq!(vec![0,1,3,3,2,2,2,1,1],
            Solution::parking_status(vec![(1,3),(2,4),(5,6),(2,8)], 9));
    }
}

