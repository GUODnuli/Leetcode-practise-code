/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 *
 * https://leetcode.cn/problems/reverse-integer/description/
 *
 * algorithms
 * Medium (35.46%)
 * Likes:    3968
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 3.6M
 * Testcase Example:  '123'
 *
 * 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。
 * 
 * 如果反转后整数超过 32 位的有符号整数的范围 [−2^31,  2^31 − 1] ，就返回 0。
 * 假设环境不允许存储 64 位整数（有符号或无符号）。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：x = 123
 * 输出：321
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：x = -123
 * 输出：-321
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：x = 120
 * 输出：21
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：x = 0
 * 输出：0
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * -2^31 
 * 
 * 
 */

// @lc code=start
// use crate::solution::Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 { return x };

        let (max_i32, min_i32) = (vec![2, 1, 4, 7, 4, 8, 3, 6, 4, 7], vec![2, 1, 4, 7, 4, 8, 3, 6, 4, 8]);
        let mut x = x;
        let mut ans:Vec<i32> = Vec::new();
        let mut negative = false;

        if x < 0 {
            negative = true;
            x = -x;
        };

        while x != 0 {
            let tmp = x % 10;
            x = x / 10;
            ans.push(tmp);
        }

        print!("{:?}", ans);

        match ans.len() {
            10 => {
                if ans[9] > 2 { return 0; };
                if negative {
                    for i in 9..=0 {
                        if ans[i] > max_i32[i] {
                            return 0;
                        }
                    }
                } else {
                    for i in 9..=0 {
                        if ans[i] > min_i32[i] {
                            return 0;
                        }
                    }
                }
            },
            _ => {}
        }

        for i in 0..ans.len() {
            x += ans[i] * i32::pow(10, (ans.len() - 1 - i) as u32);
        }

        if negative {
            x = -x;
        }

        x
    }
}
// @lc code=end
