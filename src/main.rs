pub struct Solution {}
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut mask = 0;
        while mask < num {
            mask = (mask << 1) | 1;
        }
        !num & mask
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
        assert_eq!(2, Solution::find_complement(5));
    }
    #[test]
    fn test_2() {
        assert_eq!(0, Solution::find_complement(0));
    }
}
