/*
    Find Duplicates in Array
    Given an array, find all the duplicate elements and return them. 
    You need to solve the problem with O(1) space complexity (i.e., without using extra arrays or hash tables).

    Implement the function `find_duplicates(nums: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the duplicate elements in the array.
    
    Hint: You can modify the input array in place to track duplicates.
*/

use std::fmt::{self, Display, Formatter};

pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut duplicates = Vec::new();
    
    for i in 0..nums.len() {
        let num = nums[i].abs() as usize;
        // Process all numbers, but only use them as indices if they're in range
        if num > 0 {
            if num <= nums.len() {
                let index = num - 1;
                if nums[index] > 0 {
                    nums[index] = -nums[index];
                } else {
                    let duplicate = num as i32;
                    if !duplicates.contains(&duplicate) {
                        duplicates.push(duplicate);
                    }
                }
            } else {
                // For numbers larger than array length, we need to check earlier occurrences
                let num_i32 = num as i32;
                if i > 0 && nums[..i].contains(&num_i32) && !duplicates.contains(&num_i32) {
                    duplicates.push(num_i32);
                }
            }
        }
    }
    
    duplicates.sort();
    duplicates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates_1() {
        let nums = vec![1, 2, 3, 4, 5, 6, 2, 3];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_find_duplicates_2() {
        let nums = vec![4, 5, 6, 7, 5, 4];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![4, 5]);
    }

    #[test]
    fn test_find_duplicates_3() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_find_duplicates_4() {
        let nums = vec![1, 1, 1, 1, 1];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_find_duplicates_5() {
        let nums = vec![10, 9, 8, 7, 6, 7, 8];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![7, 8]);
    }
}
