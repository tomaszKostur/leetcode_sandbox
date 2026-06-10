use std::vec;

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    for (idx, val) in nums.iter().enumerate() {
        if val >= &target {
            return idx as i32;
        }
    }
    return nums.len() as i32;
}

pub fn test1() {
    assert!(search_insert(vec![1, 3, 5, 6], 5) == 2);
    assert!(search_insert(vec![1, 3, 5, 6], 2) == 1);
    assert!(search_insert(vec![1, 3, 5, 6], 7) == 4);
}
