/// Time Complexity:  O(N)
/// Space Complexity: O(N)
/// todo: solve with O(1) space complexity
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![1; n];
    let mut left = vec![1; n];
    let mut right = vec![1; n];
    for i in 0..n - 1 { left[i + 1] *= left[i] * nums[i]; }
    for i in (1..n).rev() { right[i - 1] *= right[i] * nums[i]; }
    for i in 0..n { result[i] = left[i] * right[i]; }
    result
}

fn main() {
    assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
}
