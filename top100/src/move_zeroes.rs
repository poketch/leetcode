
// 283 - Move Zeroes
// https://leetcode.com/problems/move-zeroes/

use crate::Solution;


impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) -> () {
        
        if nums.len() == 1 { return; }

        let mut zeroes = Vec::<i32>::new();

        while let Some(index) = nums.iter().position(|&e| e == 0) {
            
            zeroes.push(nums.remove(index));

        }

        nums.extend(zeroes);
    }

    // 0 ms solution - Basically my logic but faster
    // Vec::remove is slow in general (O(n)) since it shuffles the vector after removing
    // here retain serve the function of while let remove, removing all elements which return true in the closure
    // Vec::with_capacity() lets you get away with never resizing the vec which is really nice
    // 
    /*
        pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zeroes = Vec::with_capacity(nums.len());
        nums.retain(|n| if *n == 0 {zeroes.push(0); false} else {true});
        nums.append(&mut zeroes);
    }

    */


}


#[cfg(test)]
mod test {

    #[test]
    fn ex1() {

        let mut vec = vec![0,1,0,3,12];
        crate::Solution::move_zeroes(&mut vec);

        assert_eq!(vec, vec![1,3,12,0,0])
    }
    
    #[test]
    fn ex2() {
        let mut vec = vec![0];
        crate::Solution::move_zeroes(&mut vec);

        assert_eq!(vec, vec![0])
    }

    #[test]
    fn ex3() {
        let mut vec = vec![0,1,4,2,0];
        crate::Solution::move_zeroes(&mut vec);

        assert_eq!(vec, vec![1,4,2,0,0])
    }
}

