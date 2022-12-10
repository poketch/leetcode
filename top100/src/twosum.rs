
// Problem 1 - Two Sum
// https://leetcode.com/problems/two-sum/

use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut map = HashMap::<i32,usize>::new(); //save space by using HashMap::with_capacity(nums.len())

        let mut res = Vec::<i32>::new();

        for (i, num) in nums.iter().enumerate() {
            
            let diff = target - num;

            
            if let Some(n) = map.get(num) {
                
                res.push(i as i32);
                res.push(*n as i32);
                break;
                
            }

            map.insert(diff, i);
            // println!("target: {}, num: {}, diff: {}, map: {:?}", target, num, diff, map);
        }
        res.sort(); // Line only necessary to make tests pass. Rust sort is O(nlogn)
        res
    }
}



#[cfg(test)]
mod test {

    #[test]
    fn ex1() {
        assert_eq!(crate::Solution::two_sum(vec![2,7,11,15], 9), [0, 1])
    }
    
    #[test]
    fn ex2() {
        assert_eq!(crate::Solution::two_sum(vec![3,2,4], 6), [1, 2])
    }
    
    #[test]
    fn ex3() {
        assert_eq!(crate::Solution::two_sum(vec![3,3], 6), [0, 1])
    }


}