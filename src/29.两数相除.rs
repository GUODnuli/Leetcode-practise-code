/*
 * @lc app=leetcode.cn id=29 lang=rust
 *
 * [29] 两数相除
 *
 * https://leetcode.cn/problems/divide-two-integers/description/
 *
 * algorithms
 * Medium (22.29%)
 * Likes:    1220
 * Dislikes: 0
 * Total Accepted:    234.5K
 * Total Submissions: 1.1M
 * Testcase Example:  '10\n3'
 *
 * 给你两个整数，被除数 dividend 和除数 divisor。将两数相除，要求 不使用 乘法、除法和取余运算。
 * 
 * 整数除法应该向零截断，也就是截去（truncate）其小数部分。例如，8.345 将被截断为 8 ，-2.7335 将被截断至 -2 。
 * 
 * 返回被除数 dividend 除以除数 divisor 得到的 商 。
 * 
 * 注意：假设我们的环境只能存储 32 位 有符号整数，其数值范围是 [−2^31,  2^31 − 1] 。本题中，如果商 严格大于 2^31 − 1
 * ，则返回 2^31 − 1 ；如果商 严格小于 -2^31 ，则返回 -2^31^ 。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: dividend = 10, divisor = 3
 * 输出: 3
 * 解释: 10/3 = 3.33333.. ，向零截断后得到 3 。
 * 
 * 示例 2:
 * 
 * 
 * 输入: dividend = 7, divisor = -3
 * 输出: -2
 * 解释: 7/-3 = -2.33333.. ，向零截断后得到 -2 。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * -2^31 <= dividend, divisor <= 2^31 - 1
 * divisor != 0
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut dividend = dividend;
        let mut divisor = divisor;
        let mut result = 0;
        let sign = if (dividend > 0) ^ (divisor > 0) { -1 } else { 1 };

        dividend = if dividend > 0 { -dividend } else { dividend };
        divisor = if divisor > 0 { -divisor } else { divisor };

        while dividend <= divisor {
            let mut temp_result = -1;
            let mut temp_divisor = divisor;
            while dividend <= (temp_divisor << 1) {
                if temp_divisor <= (std::i32::MIN >> 1) { break; }
                temp_result = temp_result << 1;
                temp_divisor = temp_divisor << 1;
            }
            dividend = dividend - temp_divisor;
            result += temp_result;
        }

        if result == std::i32::MIN {
            if sign == 1 {
                return std::i32::MAX;
            } else {
                return std::i32::MIN;
            }
        } else {
            return -sign * result;
        }
    }
}
// @lc code=end

