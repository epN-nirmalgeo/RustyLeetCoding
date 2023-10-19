/*
Given a binary array nums, return the maximum number of consecutive 1's in the array.

 

Example 1:

Input: nums = [1,1,0,1,1,1]
Output: 3
Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.
Example 2:

Input: nums = [1,0,1,1,0,1]
Output: 2
 


*/
use std::cmp::max;
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut maxx: i32 = 0;
        let mut cnt: i32 = 0;
        for num in &nums {
            match num {
                1 => {
                    cnt = cnt + 1;
                }
                _ => {
                    maxx = max(cnt, maxx);
                    cnt = 0;
                }
            }
        }
        maxx = max(cnt, maxx);
        maxx
    }
}