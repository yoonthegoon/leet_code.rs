/// Time Complexity:  O(n)
/// Space Complexity: O(1)
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    for j in 0..nums.len() {
        if nums[j] != nums[i] {
            i += 1;
            nums[i] = nums[j];
        }
    }
    i as i32 + 1
}

fn main() {
    assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
    assert_eq!(remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]), 5);
}
