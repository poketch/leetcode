
use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        
        let mut lowest = 1_000_000; // price is 1e4 at most

        let mut profit = 0; 

        for price in prices {

            if price < lowest {
                
                lowest = price;
                continue;
            }
            if price - lowest > profit {
                profit = price - lowest;
            }
        }

        profit
    }
}


#[cfg(test)]
mod test {

    #[test]
    fn ex1() {
        assert_eq!(crate::Solution::max_profit(vec![7,1,5,3,6,4]), 5)
    }
    
    #[test]
    fn ex2() {
        assert_eq!(crate::Solution::max_profit(vec![7,6,4,3,1]), 0)
    }

    #[test]
    fn ex3() {
        assert_eq!(crate::Solution::max_profit(vec![2,4,1]), 2)
    }

    #[test]
    fn ex4() {
        assert_eq!(crate::Solution::max_profit(vec![1]), 0)
    }
}