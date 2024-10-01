/// Time Complexity:  O(N)
/// Space Complexity: O(1)
fn majority_element(nums: Vec<i32>) -> i32 {
    let mut frequency = 0;
    let mut majority_element = 0;
    for num in nums {
        if frequency == 0 {
            frequency = 1;
            majority_element = num;
            continue;
        }
        if num == majority_element { frequency += 1; } else { frequency -= 1; }
    }
    majority_element
}

fn main() {
    assert_eq!(majority_element(vec![3, 2, 3]), 3);
    assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
