/// Time Complexity:  O(M + N)
/// Space Complexity: O(1)
fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m as usize;
    let mut n = n as usize;
    for i in (0..m + n).rev() {
        match (m, n) {
            (_, 0) => break,
            (0, _) => {
                nums1[i] = nums2[n - 1];
                n -= 1;
            }
            _ => match nums1[m - 1] > nums2[n - 1] {
                true => {
                    nums1[i] = nums1[m - 1];
                    m -= 1;
                }
                false => {
                    nums1[i] = nums2[n - 1];
                    n -= 1;
                }
            }
        }
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;
    merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

    let mut nums1 = vec![1];
    let m = 1;
    let mut nums2 = vec![];
    let n = 0;
    merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1]);

    let mut nums1 = vec![0];
    let m = 0;
    let mut nums2 = vec![1];
    let n = 1;
    merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1]);
}
