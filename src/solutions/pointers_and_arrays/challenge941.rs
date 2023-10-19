/*
941. Valid Mountain Array

Given an array of integers arr, return true if and only if it is a valid mountain array.

Recall that arr is a mountain array if and only if:

arr.length >= 3
There exists some i with 0 < i < arr.length - 1 such that:
arr[0] < arr[1] < ... < arr[i - 1] < arr[i] 
arr[i] > arr[i + 1] > ... > arr[arr.length - 1]

*/

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {

        let mut index = 0;
        let n = arr.len();

        while index + 1 < n && arr[index] < arr[index+1] {
            index += 1;
        }

        if index == 0 || index == n - 1 {
            return false;
        }
        
        while index + 1 < n && arr[index] > arr[index+1] {
            index += 1;
        }

        return index == n-1;
    }
}