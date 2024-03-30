/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 *
 * https://leetcode.cn/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (43.90%)
 * Likes:    4403
 * Dislikes: 0
 * Total Accepted:    1.8M
 * Total Submissions: 4M
 * Testcase Example:  '"()"'
 *
 * 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
 * 
 * 有效字符串需满足：
 * 
 * 
 * 左括号必须用相同类型的右括号闭合。
 * 左括号必须以正确的顺序闭合。
 * 每个右括号都有一个对应的相同类型的左括号。
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "()"
 * 输出：true
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "()[]{}"
 * 输出：true
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = "(]"
 * 输出：false
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 10^4
 * s 仅由括号 '()[]{}' 组成
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let mut stack = Vec::new();
        for i in s {
            if i == '(' || i == '{' || i == '[' {
                stack.push(i);
            } else {
                match i {
                    ')' => {
                        let c = stack.pop();
                        if c != Some('(') {
                            return false;
                        }
                    }
                    '}' => {
                        let c = stack.pop();
                        if c != Some('{') {
                            return false;
                        }
                    }
                    ']' => {
                        let c = stack.pop();
                        if c != Some('[') {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
        }

        if stack.len() != 0 {
            return false;
        }

        true
    }
}
// @lc code=end

