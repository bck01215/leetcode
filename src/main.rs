pub struct Solution {}
impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut x = num1;
        let nbits1 = num1.count_ones();
        let nbits2 = num2.count_ones();
        if nbits1 > nbits2 {
            for _ in nbits2..nbits1 {
                x &= x - 1;
            }
        } else if nbits1 < nbits2 {
            for _ in nbits1..nbits2 {
                x |= x + 1;
            }
        }
        x
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
        assert_eq!(3, Solution::minimize_xor(3, 5));
    }
    #[test]
    fn test_2() {
        assert_eq!(3, Solution::minimize_xor(1, 12));
    }
}
