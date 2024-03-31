/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 *
 * https://leetcode.cn/problems/generate-parentheses/description/
 *
 * algorithms
 * Medium (77.63%)
 * Likes:    3543
 * Dislikes: 0
 * Total Accepted:    818.7K
 * Total Submissions: 1.1M
 * Testcase Example:  '3'
 *
 * 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 3
 * 输出：["((()))","(()())","(())()","()(())","()()()"]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 1
 * 输出：["()"]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= n <= 8
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        generate("".to_string(), n, n, &mut result);
        result
    }
}

pub fn generate(s: String, left: i32, right: i32, result: &mut Vec<String>) {
    if left == 0 && right == 0 {
        result.push(s);
        return;
    }
    if left > 0 {
        generate(s.clone() + "(", left - 1, right, result);
    }
    if right > left {
        generate(s + ")", left, right - 1, result);
    }
}
// @lc code=end

