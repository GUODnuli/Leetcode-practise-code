/*
 * @lc app=leetcode.cn id=38 lang=rust
 *
 * [38] 外观数列
 *
 * https://leetcode.cn/problems/count-and-say/description/
 *
 * algorithms
 * Medium (60.93%)
 * Likes:    1087
 * Dislikes: 0
 * Total Accepted:    371.2K
 * Total Submissions: 609.3K
 * Testcase Example:  '1'
 *
 * 「外观数列」是一个数位字符串序列，由递归公式定义：
 * 
 * 
 * countAndSay(1) = "1"
 * countAndSay(n) 是 countAndSay(n-1) 的行程长度编码。
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 行程长度编码（RLE）是一种字符串压缩方法，其工作原理是通过将连续相同字符（重复两次或更多次）替换为字符重复次数（运行长度）和字符的串联。例如，要压缩字符串
 * "3322251" ，我们将 "33" 用 "23" 替换，将 "222" 用 "32" 替换，将 "5" 用 "15" 替换并将 "1" 用 "11"
 * 替换。因此压缩后字符串变为 "23321511"。
 * 
 * 给定一个整数 n ，返回 外观数列 的第 n 个元素。
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 4
 * 
 * 输出："1211"
 * 
 * 解释：
 * 
 * countAndSay(1) = "1"
 * 
 * countAndSay(2) = "1" 的行程长度编码 = "11"
 * 
 * countAndSay(3) = "11" 的行程长度编码 = "21"
 * 
 * countAndSay(4) = "21" 的行程长度编码 = "1211"
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 1
 * 
 * 输出："1"
 *
 * 解释：
 * 
 * 这是基本情况。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= n <= 30
 * 
 * 
 * 
 * 进阶：你能迭代解决该问题吗？
 */

// @lc code=start
use crate::solution::Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = vec!["1".to_string()];

        for i in 0 as usize..(n - 1) as usize {
            let curr = result[i].clone();
            let mut curr_answer = String::new();
            let mut times = 0;
            let mut char_iter = curr.chars().peekable();

            while let Some(c) = char_iter.next() {
                if let Some(&next) = char_iter.peek() {
                    if c == next {
                        times += 1;
                    } else {
                        times += 1;
                        curr_answer.push_str(&times.to_string());
                        curr_answer.push(c);
                        times = 0;
                    }
                } else {
                    times += 1;
                        curr_answer.push_str(&times.to_string());
                    curr_answer.push(c);
                    times = 0;
                }
            }
            result.push(curr_answer);
        }
        result.pop().unwrap()
    }
}
// @lc code=end

