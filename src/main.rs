pub struct Solution {}
impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let length = piles.len();
        let mut dp_table = vec![vec![0; length+1]; length];
        let mut suffix_sum = vec![0; length];
        suffix_sum[length - 1] = piles[length - 1];

        for i in (0..length - 1).rev() {
            suffix_sum[i] = suffix_sum[i + 1] + piles[i];
        }
        for i in (0..length).rev() {
            for m in 1..=length {
                if i + 2 * m >= length {
                    dp_table[i][m] = suffix_sum[i];
                }
                else {
                    for x in 1..=2*m{
                        dp_table[i][m] = dp_table[i][m].max(suffix_sum[i] - dp_table[i + x][m.max(x)]);
                    }
                }
                
            }
        }

        return dp_table[0][1];
      
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
        assert_eq!(10, Solution::stone_game_ii(vec![2,7,9,4,4]));
    }
    #[test]
    fn test_2() {
        assert_eq!(104, Solution::stone_game_ii(vec![1,2,3,4,5,100]));
    }
}
