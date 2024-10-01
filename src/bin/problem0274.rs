/// Time Complexity:  O(N)
/// Space Complexity: O(N)
fn h_index(citations: Vec<i32>) -> i32 {
    let len = citations.len();
    let mut index = 0;
    let mut indices = vec![0; len + 1];
    for citation in citations {
        let citation = citation as usize;
        if citation >= len  +1 {
            index += 1;
            continue;
        }
        indices[citation] += 1;
    }
    for i in (0..indices.len()).rev() {
        index += indices[i];
        if index >= i { return i as i32; }
    }
    0
}

fn main() {
    assert_eq!(h_index(vec![3, 0, 6, 1, 5]), 3);
    assert_eq!(h_index(vec![1, 3, 1]), 1);
}
