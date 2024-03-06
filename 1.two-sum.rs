/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans = Vec::new();
        let iter = nums.iter();

        for (index, element) in iter.enumerate() {
            if map.contains_key(&(target - element)) {
                ans = vec![
                    map.get(&(target - element))
                        .unwrap()
                        .clone(),
                    index.clone() as i32
                ]
            } else {
                map.insert(element, index.clone() as i32);
            }
        }
    }
}
// @lc code=end
