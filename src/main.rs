use std::collections::HashSet;
use std::iter::FromIterator;
pub struct Solution {}
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        HashSet::<i32>::from_iter(nums1)
            .intersection(&HashSet::<i32>::from_iter(nums2))
            .map(|&n| n)
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            vec![0],
            Solution::intersection(vec![0, 1, 1, 0], vec![2, 9, 0])
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            vec![9, 4],
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4])
        );
    }
}
