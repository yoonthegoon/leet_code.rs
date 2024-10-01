/// Time Complexity:  O(N)
/// Space Complexity: O(1)
fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() == 1 { return true; }
    let mut juice = nums[0];
    if juice == 0 { return false; }
    for i in 1..nums.len() - 1 {
        juice -= 1;
        let jump = nums[i];
        if jump > juice { juice = jump; }
        if juice == 0 { return false; }
    }
    true
}

fn main() {
    assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
    assert_eq!(can_jump(vec![0, 1]), false);
    assert_eq!(can_jump(vec![0]), true);
    assert_eq!(can_jump(vec![2, 0, 0]), true);
}
