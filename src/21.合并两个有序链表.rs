/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 *
 * https://leetcode.cn/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (66.35%)
 * Likes:    3476
 * Dislikes: 0
 * Total Accepted:    1.7M
 * Total Submissions: 2.5M
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：l1 = [1,2,4], l2 = [1,3,4]
 * 输出：[1,1,2,3,4,4]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：l1 = [], l2 = []
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：l1 = [], l2 = [0]
 * 输出：[0]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 两个链表的节点数目范围是 [0, 50]
 * -100 
 * l1 和 l2 均按 非递减顺序 排列
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
  fn _new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

use crate::solution::Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let mut dummy = Some(Box::new(ListNode{ val: 0, next: None }));
        let (mut ptr1, mut ptr2): (&mut Option<Box<ListNode>>, Option<Box<ListNode>>) = (&mut None, None);

        let (a, b) = (list1.as_ref().unwrap().val, list2.as_ref().unwrap().val);
        if a <= b {
          dummy.as_mut().unwrap().next = list1;
          ptr1 = &mut dummy.as_mut().unwrap().next;
          ptr2 = list2;
        } else {
          dummy.as_mut().unwrap().next = list2;
          ptr1 = &mut dummy.as_mut().unwrap().next;
          ptr2 = list1;
        }

        while ptr1.as_ref().unwrap().next.is_some() && ptr2.as_ref().is_some() {
          let (ptr1_val, ptr2_val) = (ptr1.as_ref().unwrap().val, ptr2.as_ref().unwrap().val);
          if ptr1_val < ptr2_val
            && ptr1.as_ref().unwrap().next.as_ref().unwrap().val <= ptr2_val {
            ptr1 = &mut ptr1.as_mut().unwrap().next;
          } else {
            let mut ptr1_next_node = &mut ptr1.clone();
            let mut ptr2_next_node = &mut ptr2.clone();

            ptr1_next_node = &mut ptr1_next_node.as_mut().unwrap().next;
            ptr2_next_node = &mut ptr2_next_node.as_mut().unwrap().next;

            ptr1.as_mut().unwrap().next = ptr2.take();
            ptr1 = &mut ptr1.as_mut().unwrap().next;
            ptr1.as_mut().unwrap().next = ptr1_next_node.take();

            ptr2 = ptr2_next_node.take();
          }
        }

        if ptr2.as_ref().is_some() {
          ptr1.as_mut().unwrap().next = ptr2.take();
        }

        dummy.unwrap().next
    }
}
// @lc code=end

