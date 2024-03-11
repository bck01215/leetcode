pub struct Solution {}
impl Solution {
    pub fn custom_sort_string(order: String, str: String) -> String {
        let order_map = order.chars().enumerate().fold([0; 26], |mut acc, (i, c)| {
            acc[c as usize - 'a' as usize] = i;
            acc
        });
        let mut s = str.into_bytes();
        s.sort_unstable_by_key(|&c| order_map[c as usize - 'a' as usize]);
        String::from_utf8(s).unwrap()
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
            "cdba".to_string(),
            Solution::custom_sort_string("cba".to_string(), "abcd".to_string())
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            "bcad".to_string(),
            Solution::custom_sort_string("bcafg".to_string(), "abcd".to_string())
        );
    }
}
