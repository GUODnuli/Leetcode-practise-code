/*
 * @lc app=leetcode.cn id=16 lang=rust
 *
 * [16] 最接近的三数之和
 *
 * https://leetcode.cn/problems/3sum-closest/description/
 *
 * algorithms
 * Medium (44.81%)
 * Likes:    1601
 * Dislikes: 0
 * Total Accepted:    549.2K
 * Total Submissions: 1.2M
 * Testcase Example:  '[-1,2,1,-4]\n1'
 *
 * 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。
 * 
 * 返回这三个数的和。
 * 
 * 假定每组输入只存在恰好一个解。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [-1,2,1,-4], target = 1
 * 输出：2
 * 解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [0,0,0], target = 1
 * 输出：0
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 3 <= nums.length <= 1000
 * -1000 <= nums[i] <= 1000
 * -10^4 <= target <= 10^4
 * 
 * 
 */

// @lc code=start
// use crate::solution::Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut result = std::i32::MAX;

        'first_loop: for i in 0..nums.len() {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                if result == target {
                    break 'first_loop;
                }
                let sum = nums[i] + nums[j] + nums[k];
                // print!("sum: {}, nums[i]: {}, nums[j]: {}, nums[k]: {}\n", sum, nums[i], nums[j], nums[k]);
                if (sum - target).abs() < (result - target).abs() {
                    result = sum;
                }
                if sum > target {
                    k -= 1;
                } else {
                    j += 1;
                }
                
            }
        }

        result
    }
}
// @lc code=end

