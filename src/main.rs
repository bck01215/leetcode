pub struct Solution {}
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        s.chars()
            .fold((0, 0i32), |(max_depth, nesting), c| {
                // Annotate nesting with i32
                match c {
                    '(' => (max_depth.max(nesting + 1), nesting + 1),
                    ')' => (max_depth, nesting.saturating_sub(1)),
                    _ => (max_depth, nesting),
                }
            })
            .0
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
        assert_eq!(3, Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()));
    }
    #[test]
    fn test_2() {
        assert_eq!(3, Solution::max_depth("(1)+((2))+(((3)))".to_string()));
    }
}
