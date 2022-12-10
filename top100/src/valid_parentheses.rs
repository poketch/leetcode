
// Problem 20 - Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/

use crate::Solution;


impl Solution {
    pub fn is_valid_slow(s: String) -> bool {
        
        let mut ans = s; // changing the param to mut would be better, but I don't want to change the function signature 
        let mut res = true;
        while ans.len() > 0 {

            if ans.len() == 1 {
                res = false
            }

            if ans.find("()").is_some() {
                ans = ans.replace("()", "");   
            }
            else if ans.find("[]").is_some() {
                ans = ans.replace("[]", "");   
            }
            else if ans.find("{}").is_some() {
                ans = ans.replace("{}", "");   
            }
            else {
                res = false;
                break;
            }
        }  
        res     
    }

    pub fn is_valid(s: String) -> bool {

        let mut stack: Vec<char> = vec![];

        for c in s.chars() {
            match c {

                '(' | '[' | '{' => stack.push(c), //add open bracket to the statck to check for valid closed brackets 

                ')' | ']' | '}' => match stack.pop() {
                    Some(t) => if !((t == '(' && c == ')') || 
                                        (t == '[' && c == ']') || 
                                        (t == '{' && c == '}')) {
                                            return false; //if there's an open bracket but it doesn't match; not valid 
                                        },
                    
                    None => { return false;}, // if there's a closed bracket and no open to match then it's not valid
                }, 
                _ => {},
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn ex1() {
        assert_eq!(crate::Solution::is_valid("()".into()), true)
    }
    
    #[test]
    fn ex2() {
        assert_eq!(crate::Solution::is_valid("()[]{}".into()), true)
    }
    
    #[test]
    fn ex3() {
        assert_eq!(crate::Solution::is_valid("(]".into()), false)
    }

    #[test]
    fn ex4() {
        assert_eq!(crate::Solution::is_valid("[{()}]".into()), true)
    }

}