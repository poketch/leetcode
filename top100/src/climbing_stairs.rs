// 70 - Climbing Stairs
// https://leetcode.com/problems/climbing-stairs/

use num_bigint::{BigInt, ToBigInt};
use crate::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return n;
        }

        let twos = n / 2;

        let init_size = if n % 2 == 0 { twos } else { twos + 1 };

        let mut res = 1.to_bigint().unwrap();

        for idx in 0..twos {
            let curr_size = init_size + idx;
            let curr_twos = twos - idx;
            let curr_ones = curr_size - curr_twos;

            res += Self::factorial(curr_size.to_bigint().unwrap())
                / (Self::factorial(curr_twos.to_bigint().unwrap()) * Self::factorial(curr_ones.to_bigint().unwrap()));
            println!(
                "curr_size: {}, fact: {}, res: {}",
                curr_size,
                Self::factorial(curr_size.to_bigint().unwrap()),
                res
            );
        }

        res.iter_u32_digits().collect::<Vec<u32>>()[0] as i32
    }

    pub fn factorial(num: BigInt) -> BigInt {
        
        let val = num.iter_u32_digits().collect::<Vec<u32>>()[0];

        match val {
            0 => 1.to_bigint().unwrap(),
            1.. => (1..=val).product(),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn ex1() {
        assert_eq!(crate::Solution::climb_stairs(2), 2)
    }

    #[test]
    fn ex2() {
        assert_eq!(crate::Solution::climb_stairs(3), 3)
    }

    #[test]
    fn ex3() {
        assert_eq!(crate::Solution::climb_stairs(4), 5)
    }

    #[test]
    fn ex4() {
        assert_eq!(crate::Solution::climb_stairs(35), 14930352)
    }

    #[test]
    fn ex5() {
        assert_eq!(crate::Solution::climb_stairs(45), 1836311903)
    }
}
