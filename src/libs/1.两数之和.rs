/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */
mod solution;
use solution::Solution;
// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] + nums[i] == target {
                    return vec![j as i32, i as i32];
                }
            }
        }
        vec![0, 0]
    }
}

// @lc code=end

#[test]
fn test_s() {
    let a = &Solution::two_sum(vec![1, 2, 3], 3)[..];
    println!("[{}, {}]", a[0], a[1]);
}
