pub struct Solution {}
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (1 << 31) % n == 0
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
        assert_eq!(true, Solution::is_power_of_two(1));
    }
    #[test]
    fn test_2() {
        assert_eq!(true, Solution::is_power_of_two(16));
    }
    #[test]
    fn test_3() {
        assert_eq!(false, Solution::is_power_of_two(3));
    }
}
