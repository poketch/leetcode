
// 136- Single Number
// https://leetcode.com/problems/single-number/

use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        
        let mut map = HashMap::<i32,i32>::new();

        for num in nums {

            if let Some(val) = map.get_mut(&num) {

                *val += 1;

            } else {

                map.insert(num, 1);

            }
        }

        *Solution::find_key_for_value(&map, 1).unwrap()

    }

    pub fn find_key_for_value<'a>(map: &'a HashMap<i32, i32>, value: i32) -> Option<&'a i32> {
        map.iter()
            .find_map(|(key, &val)| if val == value { Some(key) } else { None })
    }

    pub fn single_number_low_mem(nums: Vec<i32>) -> i32 {
        
        // this solution works by using bitwise XOR operation.
        // XORing result with the same value twice will result it going back to the original i.e.
        // a ^ 10 ^ 10 = a
        // thus only a (the single number) will remain after the O(n) operation
       
        let mut result = 0i32;
        for i in 0..nums.len() {
            result ^= nums[i];
        }
        result
    }



}


#[cfg(test)]
mod test {

    #[test]
    fn ex1() {
        assert_eq!(crate::Solution::single_number(vec![2,2,1]), 1)
    }
    
    #[test]
    fn ex2() {
        assert_eq!(crate::Solution::single_number(vec![4,1,2,1,2]), 4)
    }
    
    #[test]
    fn ex3() {
        assert_eq!(crate::Solution::single_number(vec![1]), 1)
    }

}