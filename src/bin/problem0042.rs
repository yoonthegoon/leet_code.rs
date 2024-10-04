pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut acc = 0;

    let mut buf = 0;
    let mut i = 0;
    for j in 0..n {
        if height[j] < height[i] {
            buf += height[i] - height[j];
        } else {
            acc += buf;
            buf = 0;
            i = j;
        }
    }

    buf = 0;
    i = n - 1;
    for j in (0..n).rev() {
        if height[j] <= height[i] {
            buf += height[i] - height[j];
        } else {
            acc += buf;
            buf = 0;
            i = j;
        }
    }

    acc
}

fn main() {
    assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    assert_eq!(trap(vec![2, 0, 2]), 2);
}
