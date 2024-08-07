/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 *
 * https://leetcode.cn/problems/first-missing-positive/description/
 *
 * algorithms
 * Hard (44.96%)
 * Likes:    2148
 * Dislikes: 0
 * Total Accepted:    407.8K
 * Total Submissions: 906.6K
 * Testcase Example:  '[1,2,0]'
 *
 * 给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
 * 请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。
 *
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,2,0]
 * 输出：3
 * 解释：范围 [1,2] 中的数字都在数组中。
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [3,4,-1,1]
 * 输出：2
 * 解释：1 在数组中，但 2 没有。
 *
 * 示例 3：
 *
 *
 * 输入：nums = [7,8,9,11,12]
 * 输出：1
 * 解释：最小的正数 1 没有出现。
 * 
 *
 *
 * 提示：
 * 
 *
 * 1 <= nums.length <= 10^5
 * -2^31 <= nums[i] <= 2^31 - 1
 *
 *
 */

// @lc code=start
// use crate::solution::Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();

        for i in 0..n {
            if nums[i] <= 0 || nums[i] > n as i32 {
                nums[i] = (n + 1) as i32;
            }
        }

        for i in 0..n {
            if nums[i] <= n as i32 {
                let index = (nums[i] - 1) as usize;
                if nums[i] == nums[index] { break; }
                let tmp = nums[index];
                nums[index] = nums[i];
                nums[i] = tmp;
            }
        }

        print
        for i in 0..n {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }

        n as i32 + 1
    }
}
// @lc code=end

