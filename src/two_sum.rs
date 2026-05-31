
use std::{collections::HashMap, iter::Map};

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut a = 0;
    loop {
        for b in a + 1..nums.len() {
            if b > nums.len() {
                return Vec::from([0, 0]);
            }
            if nums[a] + nums[b] == target {
                return Vec::from([a as i32, b as i32]);
            }
        }
        a += 1;
    }
}

pub fn test1() {
    let res = two_sum(vec![3, 2, 4], 6);
    assert_eq!(res, vec![1, 2]);
}

pub fn two_sum_hash_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        let expected = target - nums[i];
        match seen.get(&expected) {
            Some(expected_index) => {
                return Vec::from([*expected_index as i32, i as i32]);
            }
            None => {
                seen.insert(nums[i], i);
            }
        }
    }
    vec![]
}

pub fn test2() {
    let res = two_sum_hash_map(vec![3, 2, 4], 6);
    assert_eq!(res, vec![1, 2]);
}
