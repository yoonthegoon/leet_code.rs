/// Time Complexity:  O(N)
/// Space Complexity: O(1)
fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let at = len - k as usize % len;
    nums[0..at].reverse();
    nums[at..len].reverse();
    nums.reverse();
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

    let mut nums = vec![-1, -100, 3, 99];
    rotate(&mut nums, 2);
    assert_eq!(nums, vec![3, 99, -1, -100]);

    let mut nums = vec![-1];
    rotate(&mut nums, 2);
    assert_eq!(nums, vec![-1]);
}
