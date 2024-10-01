/// Time Complexity:  O(n)
/// Space Complexity: O(1)
fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    for j in 0..nums.len() {
        if nums[j] != val {
            nums[i] = nums[j];
            i += 1;
        }
    }
    i as i32
}

fn main() {
    assert_eq!(remove_element(&mut vec![3, 2, 2, 3], 3), 2);
    assert_eq!(remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5);
    assert_eq!(remove_element(&mut vec![4, 5], 4), 1);
}
