/*
 * @lc app=leetcode.cn id=32 lang=rust
 *
 * [32] 最长有效括号
 *
 * https://leetcode.cn/problems/longest-valid-parentheses/description/
 *
 * algorithms
 * Hard (37.83%)
 * Likes:    2479
 * Dislikes: 0
 * Total Accepted:    439K
 * Total Submissions: 1.2M
 * Testcase Example:  '"(()"'
 *
 * 给你一个只包含 '(' 和 ')' 的字符串，找出最长有效（格式正确且连续）括号子串的长度。
 * 
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "(()"
 * 输出：2
 * 解释：最长有效括号子串是 "()"
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = ")()())"
 * 输出：4
 * 解释：最长有效括号子串是 "()()"
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = ""
 * 输出：0
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= s.length <= 3 * 10^4
 * s[i] 为 '(' 或 ')'
 * 
 * 
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;
use std::cmp::max;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut dp = vec![0; s.len()];
        let mut max_len = 0;
        let s = s.chars().collect::<Vec<char>>();

        for i in 1..s.len() {
            if s[i] == ')' {
                if s[i - 1] == '(' {
                    if i >= 2 {
                        dp[i] = dp[i - 2] + 2;
                    } else {
                        dp[i] = 2;
                    }
                } else if i - dp[i - 1] > 0 && s[i - dp[i - 1] - 1] == '(' {
                    dp[i] = dp[i - 1] + 2;
                    if i - dp[i - 1] >= 2 {
                        dp[i] += dp[i - dp[i - 1] - 2];
                    }
                }
                max_len = max(max_len, dp[i]);
            }
        }
        max_len as i32
    }
}
// @lc code=end

