pub struct Solution {}
impl Solution {
    /// Use common left most bits by appending 0.
    ///
    ///Time Complexity: O(nlogn)
    ///Space Complexity: O(1)
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut left = left;
        let mut right = right;
        let mut count = 0;
        while left != right {
            left >>= 1;
            right >>= 1;
            count += 1
        }
        left << count
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
        assert_eq!(0, Solution::range_bitwise_and(0, 0));
    }
    #[test]
    fn test_2() {
        assert_eq!(4, Solution::range_bitwise_and(5, 7));
    }
    #[test]
    fn test_3() {
        assert_eq!(0, Solution::range_bitwise_and(0, 214783647));
    }
}
