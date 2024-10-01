/// Time Complexity: O(n^2)
/// Space Complexity: O(1)
fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    for i in 0..nums.len() {
        if i >= nums.len() { break; }
        while nums[i] == val {
            nums.remove(i);
            if nums.len() == i { break; }
        }
    }
    nums.len() as i32
}

fn main() {
    assert_eq!(remove_element(&mut vec![3, 2, 2, 3], 3), 2);
    assert_eq!(remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5);
    assert_eq!(remove_element(&mut vec![4, 5], 4), 1);
}
