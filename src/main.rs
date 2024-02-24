use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut flight_map = HashMap::new();
        for flight_vec in flights {
            let inner_map = flight_map.entry(flight_vec[0]).or_insert(HashMap::new());
            inner_map.insert(flight_vec[1], flight_vec[2]);
        }
        println!("{:?}", flight_map);
        let mut heap: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();
        heap.push(Reverse((0 as i32, src, k + 1)));
        while heap.len() > 0 {
            let Reverse((p, i, k)) = heap.pop().unwrap();
            println!("{}, {}, {}", p, i, k);
            if i == dst {
                return p;
            }
            if k > 0 {
                if let Some(inner_map) = flight_map.get(&i) {
                    for (&next_i, &cost) in inner_map.iter() {
                        heap.push(Reverse((p + cost, next_i, k - 1)))
                    }
                }
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
        assert_eq!(
            700,
            Solution::find_cheapest_price(
                4,
                vec![
                    vec![0, 1, 100],
                    vec![1, 2, 100],
                    vec![2, 0, 100],
                    vec![1, 3, 600],
                    vec![2, 3, 200]
                ],
                0,
                3,
                1
            )
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            200,
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                1
            )
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            500,
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                0
            )
        );
    }
    #[test]
    fn test_4() {
        assert_eq!(
            500,
            Solution::find_cheapest_price(
                5,
                vec![
                    vec![4, 1, 1],
                    vec![1, 2, 3],
                    vec![0, 3, 2],
                    vec![0, 4, 10],
                    vec![3, 1, 1],
                    vec![1, 4, 3]
                ],
                2,
                1,
                1
            )
        );
    }
}
