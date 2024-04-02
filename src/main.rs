use std::collections::{HashMap, HashSet};

pub struct Solution {}
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.into_bytes();
        let t = t.into_bytes();
        let mut map: HashMap<u8, u8> = HashMap::with_capacity(s.len());
        for index in 0..s.len() {
            let val = map.entry(s[index]).or_insert(t[index]);
            if *val != t[index] {
                return false;
            }
        }

        map.values().collect::<HashSet<_>>().len() == map.keys().collect::<HashSet<_>>().len()
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
            true,
            Solution::is_isomorphic("egg".to_string(), "add".to_string())
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            false,
            Solution::is_isomorphic("foo".to_string(), "bar".to_string())
        );
    }
}
