pub struct Solution {}
impl Solution {
    /// Store relationships people and judge and compare to best case.
    ///
    ///Time Complexity: O(n)
    ///Space Complexity: O(n)
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut trust_vec = vec![vec![0; 2]; n as usize];
        for val in trust {
            trust_vec[val[0] as usize - 1][0] += 1; // trust given
            trust_vec[val[1] as usize - 1][1] += 1; // trust received
        }

        for (index, judge_candidate) in trust_vec.iter().enumerate() {
            if judge_candidate[0] == 0 && judge_candidate[1] == n - 1 {
                return (index + 1) as i32;
            }
        }
        -1
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
        assert_eq!(2, Solution::find_judge(2, vec![vec![1, 2]]));
    }
    #[test]
    fn test_2() {
        assert_eq!(3, Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]));
    }
    #[test]
    fn test_3() {
        assert_eq!(
            -1,
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![1, 3]])
        );
    }
}
