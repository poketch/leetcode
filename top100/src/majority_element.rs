// 169 - Majority Element
// https://leetcode.com/problems/majority-element/

use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        
        if nums.len() == 1 || nums.len() == 2 {
            return nums[0];
        }   
        
        let mut map = HashMap::<i32, i32>::new();

        for num in nums {
            if let Some(val) = map.get_mut(&num) {
                *val += 1;
            } else {
                map.insert(num, 1);
            }
        }
        
        let max = map.values().max().unwrap();

        *Solution::find_key_for_value(&map, *max).unwrap()
    }

    // fn find_key_for_value<'a>(map: &'a HashMap<i32, i32>, value: i32) -> Option<&'a i32> {
    //     map.iter()
    //         .find_map(|(key, &val)| if val == value { Some(key) } else { None })
    // }
}

#[cfg(test)]
mod test {

    #[test]
    fn ex1() {
        assert_eq!(crate::Solution::majority_element(vec![3, 2, 3]), 3)
    }

    #[test]
    fn ex2() {
        assert_eq!(
            crate::Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]),
            2
        )
    }
}
