/*

977. Squares of a Sorted Array
Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.

 

Example 1:

Input: nums = [-4,-1,0,3,10]
Output: [0,1,9,16,100]
Explanation: After squaring, the array becomes [16,1,0,9,100].
After sorting, it becomes [0,1,9,16,100].
Example 2:

Input: nums = [-7,-3,2,3,11]
Output: [4,9,9,49,121]
 


*/

// Two pointer approach
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len(); // usize
        let (mut left, mut right, mut result) = (0, n-1, vec![0; n]);
        let mut index = (n - 1) as i32;
        for index in (0..n).rev() {
            let left_val = (nums[left]*nums[left]).abs();
            let right_val = (nums[right]*nums[right]).abs();

            if left_val > right_val {
                result[index] = left_val;
                left += 1;
            } else {
                result[index] = right_val;
                right -= 1;
            }
        }

        result
    }
}