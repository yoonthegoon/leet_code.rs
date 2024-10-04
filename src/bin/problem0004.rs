/// Time Complexity:  O(M + N)
/// Space Complexity: O(1)
/// todo: possibly O(log(M + N)) time complexity
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();

    let mut i = 0;
    let mut j = 0;

    let mut k = (m + n) as f64 / 2.0 + 0.5;

    let mut min;
    let mut med = f64::MIN;

    // todo: clean up these nested conditionals
    while k > 0.0 {
        if i < m && j < n {
            if nums1[i] < nums2[j] {
                min = nums1[i] as f64;
                i += 1;
            } else {
                min = nums2[j] as f64;
                j += 1;
            }
        } else if i < m {
            min = nums1[i] as f64;
            i += 1;
        } else {
            min = nums2[j] as f64;
            j += 1;
        }
        k -= 1.0;
        if k < 0.0 {
            med = (med + min) / 2.0;
        } else if k < 1.0 {
            med = min;
        }
    }
    med
}

fn main() {
    assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
}
