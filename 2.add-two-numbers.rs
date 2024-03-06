/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 *
 * https://leetcode.com/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (42.15%)
 * Likes:    29840
 * Dislikes: 5844
 * Total Accepted:    4.3M
 * Total Submissions: 10.1M
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order, and each of their nodes
 * contains a single digit. Add the two numbers and return the sum as a linked
 * list.
 * 
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in each linked list is in the range [1, 100].
 * 0 <= Node.val <= 9
 * It is guaranteed that the list represents a number that does not have
 * leading zeros.
 * 
 * 
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        let mut prev1 = Some(Box::new(ListNode{ val: 0, next: l1 }));
        let mut prev2 = Some(Box::new(ListNode{ val: 0, next: l2 }));
        loop {
            let mut node1 = prev1.as_mut().unwrap();
            let mut node2 = prev2.as_mut().unwrap();
            let (val,carry) = add(node1.val, node2.val, 0);
            node1.val = val;
            if node1.next.is_None() || node2.next.is_None() {
                
            }
        }
        return prev1;
    }
}

pub fn add(num1: i32, num2: i32, carry: i32) -> (i32, i32) {
    let sum = num1 + num2 + carry;
    return (sum % 10, sum / 10);
}
// @lc code=end

