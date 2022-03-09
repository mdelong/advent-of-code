pub mod sonar_sweep {
    use std::collections::VecDeque;

    pub fn count_increase(list: &Vec<i32>) -> i64 {
        let mut count: i64 = 0;
        let mut prev = i32::MAX;

        for &item in list {
            if item > prev {
                count += 1;
            }
            prev = item;
        }
        count
    }

    pub fn count_group_increase(list: &Vec<i32>, group_size: i32) -> i64 {
        let mut count: i64 = 0;
        let mut cur_sum = 0;
        let mut queue = VecDeque::new();

        for item in list {
            queue.push_back(item);
            cur_sum += item;

            if queue.len() as i32 > group_size {
                let prev_sum = cur_sum - item;
                cur_sum -= queue.pop_front().unwrap();
                if cur_sum > prev_sum {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increase() {
        assert_eq!(3, sonar_sweep::count_increase(&vec![1, 2, 4, 1, 5, 3]));
    }

    #[test]
    fn test_count_increase_empty() {
        assert_eq!(0, sonar_sweep::count_increase(&vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn test_count_increase_decrease() {
        assert_eq!(0, sonar_sweep::count_increase(&vec![]));
    }

    #[test]
    fn test_group_count_increase() {
        let nums = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, sonar_sweep::count_group_increase(&nums, 3));
    }
}
