/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 *
 * https://leetcode.cn/problems/median-of-two-sorted-arrays/description/
 *
 * algorithms
 * Hard (42.01%)
 * Likes:    7032
 * Dislikes: 0
 * Total Accepted:    1.1M
 * Total Submissions: 2.6M
 * Testcase Example:  '[1,3]\n[2]'
 *
 * 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
 * 
 * 算法的时间复杂度应该为 O(log (m+n)) 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums1 = [1,3], nums2 = [2]
 * 输出：2.00000
 * 解释：合并数组 = [1,2,3] ，中位数 2
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums1 = [1,2], nums2 = [3,4]
 * 输出：2.50000
 * 解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
 * 
 * 
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * nums1.length == m
 * nums2.length == n
 * 0 <= m <= 1000
 * 0 <= n <= 1000
 * 1 <= m + n <= 2000
 * -10^6 <= nums1[i], nums2[i] <= 10^6
 * 
 * 
 */

// @lc code=start
use crate::solution::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        let (mut index1, mut index2) = (0, 0);
        let (mut mid1, mut mid2) = (0, 0);

        for _ in 0..=total_len / 2 {
            mid1 = mid2;
            if index1 < nums1.len() && (index2 == nums2.len() || nums1[index1] < nums2[index2]) {
                mid2 = nums1[index1];
                index1 += 1;
            } else {
                mid2 = nums2[index2];
                index2 += 1;
            }
        }

        if total_len % 2 == 0 {
            (mid1 + mid2) as f64 / 2.0
        } else {
            mid2 as f64
        }
    }
}
// @lc code=end

