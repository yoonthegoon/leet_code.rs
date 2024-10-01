/// Time Complexity: O(n)
/// Space Complexity: O(1)
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    for j in 0..nums.len() {
        if i < 2 || nums[j] > nums[i - 2] {
            nums[i] = nums[j];
            i += 1;
        }
    }
    println!("{:?}", nums);
    i as i32
}

fn main() {
    assert_eq!(remove_duplicates(&mut vec![1, 1, 1, 2, 2, 3]), 5);
    assert_eq!(remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]), 7);
}
