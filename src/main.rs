pub struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace()
            .collect::<Vec<&str>>()
            .last()
            .unwrap_or(&"")
            .len() as i32
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
        assert_eq!(5, Solution::length_of_last_word("Hello World".to_string()));
    }
    #[test]
    fn test_2() {
        assert_eq!(
            4,
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string())
        );
    }
}
