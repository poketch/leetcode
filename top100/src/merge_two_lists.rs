// 21 - Merge Two Sorted Lsits
// https://leetcode.com/problems/merge-two-sorted-lists/

// there is a faster solution here using mem::swap() and recursion

use crate::ListNode;
use crate::Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {

        let mut res: Vec<i32> = vec![];
        let mut l1 = list1;
        let mut l2 = list2;

        if let Some(_) = l1 {

            if l2.is_none() {
                return l1;
            }
    
            while l1.is_some() && l2.is_some() {
                let l1_val = l1.as_ref().unwrap().val;
                let l2_val = l2.as_ref().unwrap().val;
    
                if l1_val < l2_val {
                    res.push(l1_val);
                    l1 = l1.unwrap().next;
                } else {
                    res.push(l2_val);
                    l2 = l2.unwrap().next;
                }
            }


            // apend the remaining values after one list is exhausted
            while let Some(node) = l1 {
                res.push(node.val);
                l1 = node.next;
            }

            while let Some(node) = l2 {
                res.push(node.val);
                l2 = node.next;
            }
        } else {
            return l2
        }
        
        Self::from_list(res)
    }

    fn from_list(array: Vec<i32>) -> Option<Box<ListNode>> {

        //attaching each node to the next by going through the list backwards
        let mut node = None;

        for i in 1..=array.len() {

            let value = array[array.len() - i];

            if i == 1 {
                node = Some(Box::new(ListNode::new(value)));
            } else {
                node = Some(Box::new(ListNode { val: value, next: node }))
            }
        }
        node
    }
}

#[cfg(test)]
mod test {
    use crate::ListNode;

    #[test]
    fn ex1() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));

        let out_list = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        };

        assert_eq!(
            crate::Solution::merge_two_lists(list1, list2),
            Some(Box::new(out_list))
        )
    }

    #[test]
    fn ex2() {
        let list1 = None;
        let list2 = None;

        assert_eq!(crate::Solution::merge_two_lists(list1, list2), None)
    }

    #[test]
    fn ex3() {
        let list1 = None;
        let list2 = Some(Box::new(ListNode::new(0)));

        assert_eq!(
            crate::Solution::merge_two_lists(list1, list2),
            Some(Box::new(ListNode::new(0)))
        )
    }

    #[test]
    fn ex4() {
        let list1 = Some(Box::new(ListNode::new(2)));
        let list2 = None;

        assert_eq!(
            crate::Solution::merge_two_lists(list1, list2),
            Some(Box::new(ListNode::new(2)))
        )
    }
}
