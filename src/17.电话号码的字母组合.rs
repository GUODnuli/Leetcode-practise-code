/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 *
 * https://leetcode.cn/problems/letter-combinations-of-a-phone-number/description/
 *
 * algorithms
 * Medium (59.27%)
 * Likes:    2782
 * Dislikes: 0
 * Total Accepted:    840.2K
 * Total Submissions: 1.4M
 * Testcase Example:  '"23"'
 *
 * 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。
 * 
 * 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
 * 
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：digits = "23"
 * 输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：digits = ""
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：digits = "2"
 * 输出：["a","b","c"]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= digits.length <= 4
 * digits[i] 是范围 ['2', '9'] 的一个数字。
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let mut queue = vec![String::new()];
        let map = [" ", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

        for digit in digits.chars() {
            let index = digit.to_digit(10).unwrap() as usize;
            let letters = map[index].chars();

            let mut next_queue = vec![];
            for letter in letters {
                for combination in &queue {
                    let mut new_combination = combination.clone();
                    new_combination.push(letter);
                    next_queue.push(new_combination);
                }
            }
            queue = next_queue;
        }
        queue
    }
}
// @lc code=end

