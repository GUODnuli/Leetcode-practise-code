/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并 K 个升序链表
 *
 * https://leetcode.cn/problems/merge-k-sorted-lists/description/
 *
 * algorithms
 * Hard (59.29%)
 * Likes:    2791
 * Dislikes: 0
 * Total Accepted:    783.5K
 * Total Submissions: 1.3M
 * Testcase Example:  '[[1,4,5],[1,3,4],[2,6]]'
 *
 * 给你一个链表数组，每个链表都已经按升序排列。
 * 
 * 请你将所有链表合并到一个升序链表中，返回合并后的链表。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：lists = [[1,4,5],[1,3,4],[2,6]]
 * 输出：[1,1,2,3,4,4,5,6]
 * 解释：链表数组如下：
 * [
 * ⁠ 1->4->5,
 * ⁠ 1->3->4,
 * ⁠ 2->6
 * ]
 * 将它们合并到一个有序链表中得到。
 * 1->1->2->3->4->4->5->6
 * 
 * 
 * 示例 2：
 * 
 * 输入：lists = []
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 输入：lists = [[]]
 * 输出：[]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * k == lists.length
 * 0 <= k <= 10^4
 * 0 <= lists[i].length <= 500
 * -10^4 <= lists[i][j] <= 10^4
 * lists[i] 按 升序 排列
 * lists[i].length 的总和不超过 10^4
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        }

        let mut lists = lists;
        let mut dummy = Some(Box::new(ListNode{next: None, val: 0})); // 这里无法通过编译
        for i in &mut lists {
            if dummy.as_ref().unwrap().next.is_none() {
                dummy.as_mut().unwrap().next = i.take();
                continue;
            }
            if i.is_some() {
                let val = dummy.as_ref().unwrap().next.as_ref().unwrap().val;
                if i.as_ref().unwrap().val < val {
                    let temp_node = dummy.as_mut().unwrap().next.clone();
                    dummy.as_mut().unwrap().next = i.take();
                    *i = temp_node;
                }
            }
        }

        if dummy.as_ref().unwrap().next.is_none() {
            return None;
        }

        for i in 1..lists.len() {
            let mut ptr1 = &mut dummy.as_mut().unwrap().next;
            let mut ptr2 = lists[i].take();

            print!("ptr1_val: {:?}\n", ptr1.as_ref().unwrap().val);

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
        }

        dummy.unwrap().next
    }
}
// @lc code=end

