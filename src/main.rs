use std::cmp::Reverse;
use std::collections::BinaryHeap;
pub struct Solution {}
impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut busy: BinaryHeap<Reverse<Vec<i64>>> = BinaryHeap::new();
        let mut meetings = meetings.clone();
        let mut available: BinaryHeap<Reverse<i64>> = (0..n).map(|x| Reverse(x as i64)).collect();
        let mut count = vec![0; n as usize];
        meetings.sort();
        for meeting in meetings {
            let start = meeting[0];
            let end = meeting[1];
            while busy.len() > 0 && busy.peek().unwrap().0[0] <= start as i64 {
                let Reverse(resp) = busy.pop().unwrap();
                let room = resp[1];
                available.push(Reverse(room));
            }
            if available.len() > 0 {
                let Reverse(room) = available.pop().unwrap();
                busy.push(Reverse(vec![end as i64, room]));
                count[room as usize] += 1;
            } else {
                let Reverse(resp) = busy.pop().unwrap();
                let time = resp[0];
                let room = resp[1];
                busy.push(Reverse(vec![time + end as i64 - start as i64, room]));
                count[room as usize] += 1;
            }
        }
        let max = count.iter().max().unwrap();
        count.iter().position(|&x| x == max.clone()).unwrap() as i32
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
        let testname: Vec<Vec<i32>> = vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]];
        assert_eq!(0, Solution::most_booked(2, testname));
    }

    #[test]
    fn test_2() {
        let testname: Vec<Vec<i32>> =
            vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]];
        assert_eq!(1, Solution::most_booked(3, testname));
    }

    #[test]
    fn test_3() {
        let testname: Vec<Vec<i32>> =
            vec![vec![0, 10], vec![1, 9], vec![2, 8], vec![3, 7], vec![4, 6]];
        assert_eq!(1, Solution::most_booked(3, testname));
    }
    #[test]
    fn test_4() {
        let testname: Vec<Vec<i32>> = vec![vec![5, 12], vec![3, 15], vec![9, 11]];
        assert_eq!(0, Solution::most_booked(3, testname));
    }
    #[test]
    fn test_6() {
        let testname: Vec<Vec<i32>> = vec![
            vec![48, 49],
            vec![31, 40],
            vec![13, 44],
            vec![26, 44],
            vec![17, 41],
            vec![2, 4],
            vec![19, 41],
            vec![42, 44],
        ];
        assert_eq!(0, Solution::most_booked(4, testname));
    }
    #[test]
    fn test_7() {
        let testname: Vec<Vec<i32>> = vec![
            vec![48, 49],
            vec![22, 30],
            vec![13, 31],
            vec![31, 46],
            vec![37, 46],
            vec![32, 36],
            vec![25, 36],
            vec![49, 50],
            vec![24, 34],
            vec![6, 41],
        ];
        assert_eq!(0, Solution::most_booked(4, testname));
    }
}
