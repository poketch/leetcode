// 35 - Search Insert Position
// https://leetcode.com/problems/search-insert-position/

use crate::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        //binary seary problem

        let mut start = 0;
        let mut end = nums.len() - 1;

        let mut mid;

        while end - start > 1 {
            
            mid = (start + end) / 2;

            if target == nums[mid] {
                return mid as i32;
            }

            if nums[mid] < target {
                start = mid + 1;
            } else {
                end = mid;
            }
        }

        println!(
            "start: {} idx: {}, end: {} idx: {}, target: {}",
            nums[start], start, nums[end], end, target
        );

        if target == nums[start] {
            return start as i32;
        }

        if target == nums[end] {
            return end as i32;
        }

        if target > nums[end] {
            return (end+1) as i32;
        }

        if nums[start] < target && target < nums[end] {
            return end as i32;
        }

        return start as i32;
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn ex1() {
        assert_eq!(crate::Solution::search_insert(vec![1, 3, 5, 6], 5), 2)
    }

    #[test]
    fn ex2() {
        assert_eq!(crate::Solution::search_insert(vec![1, 3, 5, 6], 2), 1)
    }

    #[test]
    fn ex3() {
        assert_eq!(crate::Solution::search_insert(vec![1, 3, 5, 6], 7), 4)
    }

    #[test]
    fn ex4() {
        assert_eq!(crate::Solution::search_insert(vec![1, 3, 5], 4), 2)
    }

    #[test]
    fn ex5() {
        assert_eq!(crate::Solution::search_insert(vec![1, 3, 5], 6), 3)
    }

    #[test]
    fn ex6() {
        assert_eq!(crate::Solution::search_insert(vec![1, 3, 5, 6], 0), 0)
    }

    #[test]
    fn ex7() {
        assert_eq!(crate::Solution::search_insert(vec![3, 5, 7, 9, 10], 8), 3)
    }

    #[test]
    fn ex8() {
        assert_eq!(crate::Solution::search_insert(vec![1,3], 3), 1)
    }
}
