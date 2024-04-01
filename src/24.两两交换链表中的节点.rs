/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
 *
 * https://leetcode.cn/problems/swap-nodes-in-pairs/description/
 *
 * algorithms
 * Medium (72.01%)
 * Likes:    2184
 * Dislikes: 0
 * Total Accepted:    819.1K
 * Total Submissions: 1.1M
 * Testcase Example:  '[1,2,3,4]'
 *
 * 给你一个链表，两两交换其中相邻的节点，并返回交换后链表的头节点。你必须在不修改节点内部的值的情况下完成本题（即，只能进行节点交换）。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：head = [1,2,3,4]
 * 输出：[2,1,4,3]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：head = []
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：head = [1]
 * 输出：[1]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 链表中节点的数目在范围 [0, 100] 内
 * 0 <= Node.val <= 100
 * 
 * 
 */

// @lc code=start
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
use crate::solution::Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;
        let mut pre_node = &mut dummy.as_mut();
        let mut ptr1 = &mut dummy.as_mut().unwrap().next;
        let mut ptr2 = &mut ptr1.clone().unwrap().next;

        // {
        //   ptr1 = &mut dummy.as_mut().unwrap().next;
        //   ptr2 = ptr1.clone().unwrap().next;
        // }

        while ptr1.as_ref().is_some() && ptr2.as_ref().is_some() {
          let mut next_node = ptr2.clone().unwrap().next;
          ptr1.as_mut().unwrap().next = next_node;
          ptr2.as_mut().unwrap().next = ptr1.take();
          pre_node.as_mut().unwrap().next = ptr2.take();

        }

        dummy.unwrap().next
    }
}
// @lc code=end

