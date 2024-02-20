pub struct Solution {}
impl Solution {
    /// Finds the missing number  in the array by taking the sum of all numbers in the array and
    /// the sum of all numbers in the range of the array. It then subtracts the two to find the
    /// missing number.
    ///
    ///Time Complexity: O(n)
    ///Space Complexity: O(1)
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let n: i32 = nums.len() as i32;
        n * (n + 1) / 2 - sum
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
        assert_eq!(2, Solution::missing_number(vec![3, 0, 1]));
    }
    #[test]
    fn test_2() {
        assert_eq!(2, Solution::missing_number(vec![0, 1]));
    }
}
